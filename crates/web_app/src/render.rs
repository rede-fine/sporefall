use web_sys::CanvasRenderingContext2d;

use crate::app::{AppState, Difficulty, GamePhase};
use crate::images::ImageCache;

const W: f64 = 960.0;
const H: f64 = 540.0;

pub fn draw(ctx: &CanvasRenderingContext2d, app: &AppState, images: &ImageCache) {
    ctx.set_fill_style_str("#1a2218");
    ctx.fill_rect(0.0, 0.0, W, H);

    match &app.phase {
        GamePhase::Menu => draw_menu(ctx, app),
        GamePhase::Playing => draw_playing(ctx, app, images),
        GamePhase::Paused => {
            draw_playing(ctx, app, images);
            draw_pause_overlay(ctx);
        }
        GamePhase::BasketFact { mushrooms, fact } => {
            draw_playing(ctx, app, images);
            draw_basket_fact(ctx, mushrooms, fact, images);
        }
        GamePhase::GameOver => draw_game_over(ctx, app),
    }
}

fn draw_menu(ctx: &CanvasRenderingContext2d, app: &AppState) {
    // Title
    ctx.set_fill_style_str("#f8f3e8");
    ctx.set_font("bold 52px Georgia");
    let _ = ctx.fill_text("\u{1f344} Sporefall", 280.0, 100.0);

    ctx.set_font("18px Georgia");
    ctx.set_fill_style_str("#a8c49a");
    let _ = ctx.fill_text("A mushroom sorting game \u{2014} learn fungi while you play!", 260.0, 140.0);

    // Difficulty selection
    ctx.set_fill_style_str("#f8f3e8");
    ctx.set_font("bold 22px Georgia");
    let _ = ctx.fill_text("Select Difficulty:", 260.0, 200.0);

    let difficulties = Difficulty::all();
    for (i, diff) in difficulties.iter().enumerate() {
        let y = 240.0 + i as f64 * 80.0;
        let selected = i == app.menu_selection;

        if selected {
            ctx.set_fill_style_str("#3a5a3c");
            ctx.fill_rect(240.0, y - 10.0, 480.0, 65.0);
            ctx.set_fill_style_str("#f5d67a");
            ctx.set_font("bold 16px Georgia");
            let _ = ctx.fill_text("\u{25b6}", 250.0, y + 24.0);
        }

        // Difficulty name
        ctx.set_fill_style_str(if selected { "#f5d67a" } else { "#f8f3e8" });
        ctx.set_font("bold 24px Georgia");
        let _ = ctx.fill_text(diff.label(), 275.0, y + 24.0);

        // Description
        ctx.set_fill_style_str("#8aaa7c");
        ctx.set_font("14px Georgia");
        let _ = ctx.fill_text(diff.description(), 275.0, y + 45.0);
    }

    // Level info
    ctx.set_fill_style_str("#6a8a6c");
    ctx.set_font("14px Georgia");
    let _ = ctx.fill_text("4 Levels: Cap Color \u{2192} Culinary Type \u{2192} Ecological Role \u{2192} Peak Season", 260.0, 490.0);

    // Controls
    ctx.set_fill_style_str("#8aaa7c");
    ctx.set_font("16px Georgia");
    let _ = ctx.fill_text("\u{2191}\u{2193} select   ENTER/SPACE start", 340.0, 520.0);
}

fn draw_playing(ctx: &CanvasRenderingContext2d, app: &AppState, images: &ImageCache) {
    let lane_count = app.game.lane_count();
    let lane_width = 160.0;
    let lane_gap = 12.0;
    let total_w = lane_count as f64 * lane_width + (lane_count as f64 - 1.0) * lane_gap;
    let lanes_x = (W - total_w) / 2.0;

    // Layout: large fall zone, small bucket zone at bottom
    let header_h = 60.0;
    let bucket_h = 80.0;
    let fall_zone_top = header_h;
    let fall_zone_height = H - header_h - bucket_h - 30.0;
    let bucket_top = fall_zone_top + fall_zone_height;
    let bucket_bottom = bucket_top + bucket_h;

    // Header
    ctx.set_fill_style_str("#f8f3e8");
    ctx.set_font("bold 16px Georgia");
    let _ = ctx.fill_text(
        &format!(
            "\u{1f344} Level {} \u{2014} {}",
            app.current_level + 1,
            app.category_mode.display_name()
        ),
        20.0, 24.0,
    );

    // Score
    ctx.set_fill_style_str("#f5d67a");
    ctx.set_font("bold 16px Georgia");
    let _ = ctx.fill_text(&format!("Score: {}", app.game.score()), W - 130.0, 24.0);

    // Basket count
    ctx.set_fill_style_str("#8aaa7c");
    ctx.set_font("13px Georgia");
    let _ = ctx.fill_text(
        &format!("\u{1f9fa} Baskets: {}", app.total_baskets),
        W - 130.0, 44.0,
    );

    // Difficulty badge
    ctx.set_fill_style_str("#5a8a5c");
    ctx.set_font("12px Georgia");
    let _ = ctx.fill_text(
        &format!("[{}]", app.difficulty.label()),
        20.0, 44.0,
    );

    // Retry queue indicator
    let retries = app.game.retry_queue().len();
    if retries > 0 {
        ctx.set_fill_style_str("#e07a5a");
        ctx.set_font("13px Georgia");
        let _ = ctx.fill_text(&format!("\u{21bb} Retry: {}", retries), 200.0, 44.0);
    }

    // Feedback message
    if let Some(msg) = app.feedback_message() {
        ctx.set_fill_style_str("#d7c486");
        ctx.set_font("14px Georgia");
        let _ = ctx.fill_text(msg, 20.0, 56.0);
    }

    // Fall zone background
    ctx.set_fill_style_str("#1e2e1c");
    ctx.fill_rect(lanes_x - 10.0, fall_zone_top, total_w + 20.0, fall_zone_height);

    // Bucket zone
    let labels = app.category_mode.labels();
    for i in 0..lane_count {
        let x = lanes_x + i as f64 * (lane_width + lane_gap);

        // Bucket background
        ctx.set_fill_style_str("#2a3a2c");
        ctx.fill_rect(x, bucket_top, lane_width, bucket_h);
        ctx.set_stroke_style_str("#4a6a4c");
        ctx.set_line_width(2.0);
        ctx.stroke_rect(x, bucket_top, lane_width, bucket_h);

        // Label
        let label = labels.get(i).copied().unwrap_or("???");
        ctx.set_fill_style_str("#a8c49a");
        ctx.set_font("bold 11px Georgia");
        let _ = ctx.fill_text(label, x + 6.0, bucket_top + 14.0);

        // Stacked mushrooms in bucket
        let lane_stack = &app.game.lanes()[i];
        for (j, m) in lane_stack.iter().enumerate() {
            let piece_h = 20.0;
            let piece_y = bucket_bottom - 4.0 - (j as f64 + 1.0) * piece_h;
            if piece_y < bucket_top + 16.0 {
                break;
            }
            draw_mushroom_item(ctx, m, x + 4.0, piece_y, 16.0, app.difficulty, images, true);
        }

        // Lane guide lines
        ctx.set_stroke_style_str("#3a4a3c");
        ctx.set_line_width(1.0);
        ctx.begin_path();
        ctx.move_to(x, fall_zone_top);
        ctx.line_to(x, bucket_top);
        ctx.move_to(x + lane_width, fall_zone_top);
        ctx.line_to(x + lane_width, bucket_top);
        ctx.stroke();
    }

    // Falling mushroom
    if let Some(mushroom) = app.game.active_mushroom() {
        let active_lane = app.game.active_lane();
        let x = lanes_x + active_lane as f64 * (lane_width + lane_gap);
        let fall_y = fall_zone_top + app.fall_progress * fall_zone_height;
        let cx = x + lane_width / 2.0;

        draw_mushroom_item(ctx, mushroom, cx - 28.0, fall_y - 28.0, 56.0, app.difficulty, images, false);

        // Show name based on difficulty
        match app.difficulty {
            Difficulty::Normal => {
                ctx.set_fill_style_str("#f8f3e8");
                ctx.set_font("bold 12px Georgia");
                let _ = ctx.fill_text(&mushroom.display_name, cx - 50.0, fall_y + 36.0);
                ctx.set_fill_style_str("#a8c49a");
                ctx.set_font("italic 10px Georgia");
                let _ = ctx.fill_text(&mushroom.latin_name, cx - 50.0, fall_y + 50.0);
            }
            Difficulty::Tricky => {
                // No names shown
            }
            Difficulty::Expert => {
                ctx.set_fill_style_str("#a8c49a");
                ctx.set_font("italic 11px Georgia");
                let _ = ctx.fill_text(&mushroom.latin_name, cx - 50.0, fall_y + 36.0);
            }
        }

        // Highlight active lane bucket
        ctx.set_stroke_style_str("#f5d67a");
        ctx.set_line_width(3.0);
        ctx.stroke_rect(x, bucket_top, lane_width, bucket_h);

        // Arrow
        ctx.set_fill_style_str("#f5d67a");
        ctx.set_font("14px Georgia");
        let _ = ctx.fill_text("\u{25bc}", cx - 5.0, bucket_top - 4.0);
    }

    // Bottom bar: controls + next level button
    ctx.set_fill_style_str("#5a7a5c");
    ctx.set_font("12px Georgia");
    let _ = ctx.fill_text(
        "\u{2190}\u{2192} move   SPACE drop   ESC pause",
        lanes_x, H - 8.0,
    );

    // Next level button (bottom right)
    ctx.set_fill_style_str("#3a5a3c");
    ctx.fill_rect(W - 140.0, H - 28.0, 130.0, 22.0);
    ctx.set_fill_style_str("#f5d67a");
    ctx.set_font("bold 12px Georgia");
    let _ = ctx.fill_text("[N] Next Level \u{25b6}", W - 135.0, H - 12.0);
}

/// Draw a single mushroom item respecting difficulty mode.
fn draw_mushroom_item(
    ctx: &CanvasRenderingContext2d,
    mushroom: &game_core::FallingMushroom,
    x: f64,
    y: f64,
    size: f64,
    difficulty: Difficulty,
    images: &ImageCache,
    compact: bool,
) {
    match difficulty {
        Difficulty::Normal | Difficulty::Tricky => {
            // Show real image
            if let Some(img) = images.get(&mushroom.image_key) {
                let _ = ctx.draw_image_with_html_image_element_and_dw_and_dh(img, x, y, size, size);
            } else {
                draw_fallback_sprite(ctx, &mushroom.image_key, x, y, size);
            }
        }
        Difficulty::Expert => {
            // Show mushroom emoji instead of image
            let emoji_size = if compact { size * 0.8 } else { size * 0.7 };
            ctx.set_font(&format!("{}px serif", emoji_size));
            let _ = ctx.fill_text("\u{1f344}", x, y + size * 0.8);
        }
    }

    // Show name next to item in compact mode (bucket) if Normal difficulty
    if compact && difficulty == Difficulty::Normal {
        ctx.set_fill_style_str("#f8f3e8");
        ctx.set_font("9px Georgia");
        let _ = ctx.fill_text(&mushroom.display_name, x + size + 3.0, y + size * 0.7);
    }
}

fn draw_basket_fact(
    ctx: &CanvasRenderingContext2d,
    mushrooms: &[game_core::FallingMushroom],
    fact: &str,
    images: &ImageCache,
) {
    // Semi-transparent overlay
    ctx.set_fill_style_str("rgba(10, 20, 10, 0.85)");
    ctx.fill_rect(0.0, 0.0, W, H);

    // Basket header
    ctx.set_fill_style_str("#f5d67a");
    ctx.set_font("bold 36px Georgia");
    let _ = ctx.fill_text("\u{1f9fa} Basket Collected!", 260.0, 80.0);

    // Show the 4 mushrooms in a row
    let start_x = (W - 4.0 * 100.0) / 2.0;
    for (i, m) in mushrooms.iter().enumerate() {
        let mx = start_x + i as f64 * 100.0;
        let my = 110.0;

        // Image
        if let Some(img) = images.get(&m.image_key) {
            let _ = ctx.draw_image_with_html_image_element_and_dw_and_dh(img, mx + 10.0, my, 70.0, 70.0);
        } else {
            draw_fallback_sprite(ctx, &m.image_key, mx + 10.0, my, 70.0);
        }

        // Name below
        ctx.set_fill_style_str("#f8f3e8");
        ctx.set_font("bold 11px Georgia");
        let _ = ctx.fill_text(&m.display_name, mx + 5.0, my + 85.0);
    }

    // Fun fact box
    ctx.set_fill_style_str("#2a3a2c");
    ctx.fill_rect(80.0, 220.0, W - 160.0, 220.0);
    ctx.set_stroke_style_str("#4a6a4c");
    ctx.set_line_width(2.0);
    ctx.stroke_rect(80.0, 220.0, W - 160.0, 220.0);

    // Wrap fact text
    ctx.set_fill_style_str("#f8f3e8");
    ctx.set_font("16px Georgia");
    let max_width = W - 220.0;
    let mut line_y = 260.0;
    for line in wrap_text(fact, 70) {
        let _ = ctx.fill_text(&line, 110.0, line_y);
        line_y += 24.0;
        if line_y > 420.0 {
            break;
        }
    }
    let _ = line_y;
    let _ = max_width;

    // Continue prompt
    ctx.set_fill_style_str("#f5d67a");
    ctx.set_font("bold 18px Georgia");
    let _ = ctx.fill_text("Press SPACE to continue", 340.0, 480.0);
}

fn draw_pause_overlay(ctx: &CanvasRenderingContext2d) {
    ctx.set_fill_style_str("rgba(0, 0, 0, 0.7)");
    ctx.fill_rect(0.0, 0.0, W, H);

    ctx.set_fill_style_str("#f8f3e8");
    ctx.set_font("bold 48px Georgia");
    let _ = ctx.fill_text("PAUSED", 370.0, 240.0);

    ctx.set_fill_style_str("#a8c49a");
    ctx.set_font("20px Georgia");
    let _ = ctx.fill_text("ESC / ENTER to resume", 340.0, 300.0);
    let _ = ctx.fill_text("SPACE to quit to menu", 340.0, 340.0);
}

fn draw_game_over(ctx: &CanvasRenderingContext2d, app: &AppState) {
    ctx.set_fill_style_str("#f8f3e8");
    ctx.set_font("bold 48px Georgia");
    let _ = ctx.fill_text("\u{1f389} All Levels Complete!", 220.0, 180.0);

    ctx.set_fill_style_str("#f5d67a");
    ctx.set_font("bold 28px Georgia");
    let _ = ctx.fill_text(&format!("Final Score: {}", app.game.score()), 340.0, 250.0);

    ctx.set_fill_style_str("#8aaa7c");
    ctx.set_font("20px Georgia");
    let _ = ctx.fill_text(
        &format!("\u{1f9fa} {} baskets collected", app.total_baskets),
        340.0, 300.0,
    );
    let _ = ctx.fill_text(
        &format!("\u{1f344} {} mushrooms in basket", app.game.basket().len()),
        340.0, 340.0,
    );

    ctx.set_font("16px Georgia");
    let _ = ctx.fill_text("Press ENTER or SPACE to return to menu", 310.0, 420.0);
}

/// Fallback colored mushroom shape when image not loaded.
fn draw_fallback_sprite(ctx: &CanvasRenderingContext2d, image_key: &str, x: f64, y: f64, size: f64) {
    let cap_color = match image_key {
        "chanterelle" => "#FFD700",
        "fly-agaric" => "#CC2200",
        "king-bolete" => "#8B4513",
        "oyster" => "#F5F5DC",
        "shiitake" => "#A0522D",
        "turkey-tail" => "#D2691E",
        "honey-fungus" => "#DAA520",
        "chaga" => "#3B2F2F",
        "cordyceps" => "#FF6347",
        "morel" => "#8B7355",
        "death-cap" => "#E8E8E8",
        "reishi" => "#B22222",
        _ => "#808080",
    };

    let r = size / 2.0;
    let cx = x + r;
    let cy = y + r * 0.6;

    ctx.set_fill_style_str(cap_color);
    ctx.begin_path();
    let _ = ctx.arc(cx, cy, r * 0.7, std::f64::consts::PI, 0.0);
    ctx.fill();

    ctx.set_fill_style_str("#F5F5DC");
    ctx.fill_rect(cx - r * 0.15, cy, r * 0.3, r * 0.7);
}

/// Simple word-wrap at approximately `max_chars` per line.
fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > max_chars && !current_line.is_empty() {
            lines.push(current_line.clone());
            current_line.clear();
        }
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    lines
}
