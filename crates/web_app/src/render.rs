use web_sys::CanvasRenderingContext2d;

use crate::app::{AppState, CenterAnimation, Difficulty, GamePhase};
use crate::catalog::Variety;
use crate::images::ImageCache;

const W: f64 = 960.0;
const H: f64 = 540.0;

pub fn draw(ctx: &CanvasRenderingContext2d, app: &AppState, images: &ImageCache) {
    ctx.set_fill_style_str("#1a2218");
    ctx.fill_rect(0.0, 0.0, W, H);

    match &app.phase {
        GamePhase::Menu => draw_menu(ctx, app),
        GamePhase::Playing => {
            draw_playing(ctx, app, images);
            draw_center_animation(ctx, app);
        }
        GamePhase::Paused => {
            draw_playing(ctx, app, images);
            draw_pause_overlay(ctx);
        }
        GamePhase::BasketFact { mushrooms, fact } => {
            draw_playing(ctx, app, images);
            draw_basket_fact(ctx, mushrooms, fact, images);
        }
        GamePhase::LevelComplete => draw_level_complete(ctx, app, images),
        GamePhase::GameOver => draw_game_over(ctx, app, images),
    }
}

fn draw_menu(ctx: &CanvasRenderingContext2d, app: &AppState) {
    // Title
    ctx.set_fill_style_str("#f8f3e8");
    ctx.set_font("bold 52px Georgia");
    let _ = ctx.fill_text("\u{1f344} Sporefall", 280.0, 80.0);

    ctx.set_font("18px Georgia");
    ctx.set_fill_style_str("#a8c49a");
    let _ = ctx.fill_text("A mushroom sorting game \u{2014} learn fungi while you play!", 260.0, 115.0);

    // Two columns: Game Mode (left) and Variety (right)
    let col_left_x = 100.0;
    let col_right_x = 530.0;
    let col_w = 340.0;

    // Left column header: Game Mode
    ctx.set_fill_style_str("#f8f3e8");
    ctx.set_font("bold 20px Georgia");
    let _ = ctx.fill_text("Game Mode:", col_left_x, 165.0);

    let difficulties = Difficulty::all();
    for (i, diff) in difficulties.iter().enumerate() {
        let y = 195.0 + i as f64 * 75.0;
        let selected = app.menu_column == 0 && i == app.menu_selection;
        let active = i == app.menu_selection;

        if active {
            ctx.set_fill_style_str(if selected { "#3a5a3c" } else { "#2a3a2c" });
            ctx.fill_rect(col_left_x - 10.0, y - 8.0, col_w, 60.0);
        }

        if selected {
            ctx.set_fill_style_str("#f5d67a");
            ctx.set_font("bold 14px Georgia");
            let _ = ctx.fill_text("\u{25b6}", col_left_x, y + 20.0);
        }

        ctx.set_fill_style_str(if active { "#f5d67a" } else { "#f8f3e8" });
        ctx.set_font("bold 20px Georgia");
        let _ = ctx.fill_text(diff.label(), col_left_x + 20.0, y + 20.0);

        ctx.set_fill_style_str("#8aaa7c");
        ctx.set_font("13px Georgia");
        let _ = ctx.fill_text(diff.description(), col_left_x + 20.0, y + 40.0);
    }

    // Right column header: Variety
    ctx.set_fill_style_str("#f8f3e8");
    ctx.set_font("bold 20px Georgia");
    let _ = ctx.fill_text("Variety:", col_right_x, 165.0);

    let varieties = Variety::all();
    for (i, var) in varieties.iter().enumerate() {
        let y = 195.0 + i as f64 * 75.0;
        let selected = app.menu_column == 1 && i == app.variety_selection;
        let active = i == app.variety_selection;

        if active {
            ctx.set_fill_style_str(if selected { "#3a5a3c" } else { "#2a3a2c" });
            ctx.fill_rect(col_right_x - 10.0, y - 8.0, col_w, 60.0);
        }

        if selected {
            ctx.set_fill_style_str("#f5d67a");
            ctx.set_font("bold 14px Georgia");
            let _ = ctx.fill_text("\u{25b6}", col_right_x, y + 20.0);
        }

        ctx.set_fill_style_str(if active { "#f5d67a" } else { "#f8f3e8" });
        ctx.set_font("bold 20px Georgia");
        let _ = ctx.fill_text(var.label(), col_right_x + 20.0, y + 20.0);

        ctx.set_fill_style_str("#8aaa7c");
        ctx.set_font("13px Georgia");
        let _ = ctx.fill_text(var.description(), col_right_x + 20.0, y + 40.0);
    }

    // Active column indicator
    let indicator_x = if app.menu_column == 0 { col_left_x + col_w / 2.0 - 30.0 } else { col_right_x + col_w / 2.0 - 30.0 };
    ctx.set_fill_style_str("#f5d67a");
    ctx.set_font("12px Georgia");
    let _ = ctx.fill_text("\u{25bc} ACTIVE \u{25bc}", indicator_x, 155.0);

    // Level info
    ctx.set_fill_style_str("#6a8a6c");
    ctx.set_font("14px Georgia");
    let _ = ctx.fill_text("4 Levels: Cap Color \u{2192} Culinary Type \u{2192} Ecological Role \u{2192} Peak Season", 200.0, 470.0);

    // Controls
    ctx.set_fill_style_str("#8aaa7c");
    ctx.set_font("16px Georgia");
    let _ = ctx.fill_text("\u{2190}\u{2192} switch column   \u{2191}\u{2193} select   ENTER/SPACE start", 230.0, 510.0);
}

fn draw_playing(ctx: &CanvasRenderingContext2d, app: &AppState, images: &ImageCache) {
    let lane_count = app.game.lane_count();
    let lane_width = 150.0;
    let lane_gap = 10.0;
    let total_w = lane_count as f64 * lane_width + (lane_count as f64 - 1.0) * lane_gap;

    // Game area is narrower to leave room for basket collection on right
    let game_area_w = total_w + 40.0;
    let lanes_x = 20.0 + (game_area_w - total_w) / 2.0;

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

    // Score & baskets in header
    ctx.set_fill_style_str("#f5d67a");
    ctx.set_font("bold 14px Georgia");
    let _ = ctx.fill_text(&format!("Score: {}", app.game.score()), 20.0, 44.0);

    ctx.set_fill_style_str("#8aaa7c");
    ctx.set_font("13px Georgia");
    let _ = ctx.fill_text(
        &format!("\u{1f9fa} Baskets: {}", app.total_baskets),
        120.0, 44.0,
    );

    // Progress: sorted X/Y
    let total_in_set = app.variety.count();
    let sorted_count = app.sorted_this_level.len();
    ctx.set_fill_style_str("#a8c49a");
    ctx.set_font("12px Georgia");
    let _ = ctx.fill_text(
        &format!("Sorted: {}/{}", sorted_count, total_in_set),
        240.0, 44.0,
    );

    // Difficulty + variety badge
    ctx.set_fill_style_str("#5a8a5c");
    ctx.set_font("12px Georgia");
    let _ = ctx.fill_text(
        &format!("[{} / {}]", app.difficulty.label(), app.variety.label()),
        20.0, 56.0,
    );

    // Feedback message
    if let Some(msg) = app.feedback_message() {
        ctx.set_fill_style_str("#d7c486");
        ctx.set_font("13px Georgia");
        let _ = ctx.fill_text(msg, 180.0, 56.0);
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
        let img_size = 56.0;

        // Draw mushroom image centered
        draw_mushroom_item(ctx, mushroom, cx - img_size / 2.0, fall_y - img_size / 2.0, img_size, app.difficulty, images, false);

        // Show name CENTERED below image based on difficulty
        match app.difficulty {
            Difficulty::Normal => {
                ctx.set_fill_style_str("#f8f3e8");
                ctx.set_font("bold 12px Georgia");
                let name_w = mushroom.display_name.len() as f64 * 7.0;
                let _ = ctx.fill_text(&mushroom.display_name, cx - name_w / 2.0, fall_y + img_size / 2.0 + 14.0);
                ctx.set_fill_style_str("#a8c49a");
                ctx.set_font("italic 10px Georgia");
                let latin_w = mushroom.latin_name.len() as f64 * 5.5;
                let _ = ctx.fill_text(&mushroom.latin_name, cx - latin_w / 2.0, fall_y + img_size / 2.0 + 28.0);
            }
            Difficulty::Tricky => {
                // No names shown
            }
            Difficulty::Expert => {
                ctx.set_fill_style_str("#a8c49a");
                ctx.set_font("italic 11px Georgia");
                let latin_w = mushroom.latin_name.len() as f64 * 5.5;
                let _ = ctx.fill_text(&mushroom.latin_name, cx - latin_w / 2.0, fall_y + img_size / 2.0 + 14.0);
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

    // Basket collection panel (right side)
    draw_basket_panel(ctx, app, images, game_area_w + 10.0);

    // Bottom bar: controls
    ctx.set_fill_style_str("#5a7a5c");
    ctx.set_font("12px Georgia");
    let _ = ctx.fill_text(
        "\u{2190}\u{2192} move   SPACE drop   ESC pause   [N] skip level",
        lanes_x, H - 8.0,
    );
}

/// Draw the collected basket panel on the right side.
fn draw_basket_panel(
    ctx: &CanvasRenderingContext2d,
    app: &AppState,
    images: &ImageCache,
    panel_x: f64,
) {
    let panel_w = W - panel_x - 5.0;
    if panel_w < 60.0 || app.collected_mushrooms.is_empty() {
        return;
    }

    // Panel background
    ctx.set_fill_style_str("#1e2e1c");
    ctx.fill_rect(panel_x, 60.0, panel_w, H - 90.0);
    ctx.set_stroke_style_str("#3a5a3c");
    ctx.set_line_width(1.0);
    ctx.stroke_rect(panel_x, 60.0, panel_w, H - 90.0);

    // Header
    ctx.set_fill_style_str("#f5d67a");
    ctx.set_font("bold 12px Georgia");
    let _ = ctx.fill_text("\u{1f9fa} Collection", panel_x + 8.0, 78.0);

    // Show collected mushrooms as small thumbnails in a grid
    let thumb_size = 28.0;
    let padding = 4.0;
    let cols = ((panel_w - 12.0) / (thumb_size + padding)).floor() as usize;

    for (i, m) in app.collected_mushrooms.iter().enumerate() {
        let col = i % cols.max(1);
        let row = i / cols.max(1);
        let mx = panel_x + 6.0 + col as f64 * (thumb_size + padding);
        let my = 88.0 + row as f64 * (thumb_size + padding + 12.0);

        if my + thumb_size > H - 30.0 {
            // Show overflow count
            ctx.set_fill_style_str("#8aaa7c");
            ctx.set_font("10px Georgia");
            let remaining = app.collected_mushrooms.len() - i;
            let _ = ctx.fill_text(&format!("+{} more", remaining), panel_x + 8.0, my + 10.0);
            break;
        }

        if let Some(img) = images.get(&m.image_key) {
            let _ = ctx.draw_image_with_html_image_element_and_dw_and_dh(img, mx, my, thumb_size, thumb_size);
        } else {
            draw_fallback_sprite(ctx, &m.image_key, mx, my, thumb_size);
        }

        // Tiny name centered below
        ctx.set_fill_style_str("#a8c49a");
        ctx.set_font("8px Georgia");
        let short_name = if m.display_name.len() > 5 {
            &m.display_name[..5]
        } else {
            &m.display_name
        };
        let name_w = short_name.len() as f64 * 4.5;
        let _ = ctx.fill_text(short_name, mx + (thumb_size - name_w) / 2.0, my + thumb_size + 9.0);
    }
}

/// Draw center screen animation effects.
fn draw_center_animation(ctx: &CanvasRenderingContext2d, app: &AppState) {
    let Some(ref anim) = app.animation else { return };

    let cx = 340.0; // center of game area
    let cy = H / 2.0;

    match anim {
        CenterAnimation::Correct { progress } => {
            let p = progress.min(1.0);
            // Green checkmark that starts small, grows, then fades
            let scale = if p < 0.3 { p / 0.3 } else { 1.0 };
            let alpha = if p > 0.5 { 1.0 - (p - 0.5) * 2.0 } else { 0.9 };
            let size = 80.0 * scale;

            ctx.save();
            ctx.set_global_alpha(alpha.max(0.0));
            ctx.set_fill_style_str("#4CAF50");
            ctx.set_font(&format!("bold {}px Georgia", size));
            let _ = ctx.fill_text("\u{2714}", cx - size * 0.3, cy + size * 0.3);
            ctx.restore();
        }
        CenterAnimation::Wrong { progress } => {
            let p = progress.min(1.0);
            // Red X that grows big then fades
            let scale = if p < 0.4 { p / 0.4 * 1.5 } else { 1.5 };
            let alpha = if p > 0.4 { 1.0 - (p - 0.4) / 0.6 } else { 0.9 };
            let size = 100.0 * scale;

            ctx.save();
            ctx.set_global_alpha(alpha.max(0.0));
            ctx.set_fill_style_str("#F44336");
            ctx.set_font(&format!("bold {}px Georgia", size));
            let _ = ctx.fill_text("\u{2718}", cx - size * 0.3, cy + size * 0.3);
            ctx.restore();
        }
        CenterAnimation::BasketCollected { progress } => {
            let p = progress.min(1.0);
            // Basket emoji that bounces up then settles
            let bounce = if p < 0.5 {
                (p * std::f64::consts::PI).sin() * 40.0
            } else {
                ((p - 0.5) * std::f64::consts::PI * 2.0).sin() * 10.0
            };
            let alpha = if p > 0.7 { 1.0 - (p - 0.7) / 0.3 } else { 1.0 };
            let size = 60.0 + p * 20.0;

            ctx.save();
            ctx.set_global_alpha(alpha.max(0.0));
            ctx.set_font(&format!("{}px serif", size));
            let _ = ctx.fill_text("\u{1f9fa}", cx - size * 0.3, cy - bounce);
            ctx.restore();
        }
    }
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
            if let Some(img) = images.get(&mushroom.image_key) {
                let _ = ctx.draw_image_with_html_image_element_and_dw_and_dh(img, x, y, size, size);
            } else {
                draw_fallback_sprite(ctx, &mushroom.image_key, x, y, size);
            }
        }
        Difficulty::Expert => {
            let emoji_size = if compact { size * 0.8 } else { size * 0.7 };
            ctx.set_font(&format!("{}px serif", emoji_size));
            let _ = ctx.fill_text("\u{1f344}", x, y + size * 0.8);
        }
    }

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

    // Show the mushrooms in a row with centered names below
    let item_w = 100.0;
    let total_items_w = mushrooms.len() as f64 * item_w;
    let start_x = (W - total_items_w) / 2.0;
    for (i, m) in mushrooms.iter().enumerate() {
        let mx = start_x + i as f64 * item_w;
        let my = 110.0;
        let img_size = 70.0;

        // Image centered in slot
        let img_x = mx + (item_w - img_size) / 2.0;
        if let Some(img) = images.get(&m.image_key) {
            let _ = ctx.draw_image_with_html_image_element_and_dw_and_dh(img, img_x, my, img_size, img_size);
        } else {
            draw_fallback_sprite(ctx, &m.image_key, img_x, my, img_size);
        }

        // Name centered below image
        ctx.set_fill_style_str("#f8f3e8");
        ctx.set_font("bold 11px Georgia");
        let name_w = m.display_name.len() as f64 * 6.0;
        let _ = ctx.fill_text(&m.display_name, mx + (item_w - name_w) / 2.0, my + img_size + 14.0);
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
    let mut line_y = 260.0;
    for line in wrap_text(fact, 70) {
        let _ = ctx.fill_text(&line, 110.0, line_y);
        line_y += 24.0;
        if line_y > 420.0 {
            break;
        }
    }

    // Continue prompt
    ctx.set_fill_style_str("#f5d67a");
    ctx.set_font("bold 18px Georgia");
    let _ = ctx.fill_text("Press SPACE to continue", 340.0, 480.0);
}

fn draw_level_complete(ctx: &CanvasRenderingContext2d, app: &AppState, images: &ImageCache) {
    // Semi-transparent overlay
    ctx.set_fill_style_str("rgba(10, 20, 10, 0.9)");
    ctx.fill_rect(0.0, 0.0, W, H);

    // Level complete banner
    ctx.set_fill_style_str("#f5d67a");
    ctx.set_font("bold 36px Georgia");
    let _ = ctx.fill_text(
        &format!("\u{1f389} Level {} Complete!", app.current_level + 1),
        260.0, 60.0,
    );

    ctx.set_fill_style_str("#a8c49a");
    ctx.set_font("18px Georgia");
    let _ = ctx.fill_text(
        &format!("All {} mushrooms sorted by {}!", app.variety.count(), app.category_mode.display_name()),
        240.0, 95.0,
    );

    // Show collected mushrooms in a grid
    ctx.set_fill_style_str("#f8f3e8");
    ctx.set_font("bold 16px Georgia");
    let _ = ctx.fill_text("\u{1f9fa} Your Collection:", 60.0, 140.0);

    let thumb_size = 50.0;
    let padding = 8.0;
    let cols = 8;
    let start_x = 60.0;
    let start_y = 160.0;

    for (i, m) in app.collected_mushrooms.iter().enumerate() {
        let col = i % cols;
        let row = i / cols;
        let mx = start_x + col as f64 * (thumb_size + padding + 30.0);
        let my = start_y + row as f64 * (thumb_size + padding + 14.0);

        if my + thumb_size > H - 60.0 {
            break;
        }

        if let Some(img) = images.get(&m.image_key) {
            let _ = ctx.draw_image_with_html_image_element_and_dw_and_dh(img, mx, my, thumb_size, thumb_size);
        } else {
            draw_fallback_sprite(ctx, &m.image_key, mx, my, thumb_size);
        }

        // Name centered below
        ctx.set_fill_style_str("#a8c49a");
        ctx.set_font("9px Georgia");
        let short = if m.display_name.len() > 10 { &m.display_name[..10] } else { &m.display_name };
        let name_w = short.len() as f64 * 5.0;
        let _ = ctx.fill_text(short, mx + (thumb_size - name_w) / 2.0, my + thumb_size + 10.0);
    }

    // Score
    ctx.set_fill_style_str("#f5d67a");
    ctx.set_font("bold 18px Georgia");
    let _ = ctx.fill_text(&format!("Score: {}", app.game.score()), 60.0, H - 40.0);

    // Continue prompt
    ctx.set_fill_style_str("#f5d67a");
    ctx.set_font("bold 18px Georgia");
    if app.current_level < 3 {
        let _ = ctx.fill_text("Press SPACE for next level \u{25b6}", 550.0, H - 40.0);
    } else {
        let _ = ctx.fill_text("Press SPACE to finish!", 550.0, H - 40.0);
    }
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

fn draw_game_over(ctx: &CanvasRenderingContext2d, app: &AppState, images: &ImageCache) {
    ctx.set_fill_style_str("rgba(10, 20, 10, 0.9)");
    ctx.fill_rect(0.0, 0.0, W, H);

    ctx.set_fill_style_str("#f8f3e8");
    ctx.set_font("bold 48px Georgia");
    let _ = ctx.fill_text("\u{1f389} All Levels Complete!", 220.0, 100.0);

    ctx.set_fill_style_str("#f5d67a");
    ctx.set_font("bold 28px Georgia");
    let _ = ctx.fill_text(&format!("Final Score: {}", app.game.score()), 340.0, 160.0);

    ctx.set_fill_style_str("#8aaa7c");
    ctx.set_font("20px Georgia");
    let _ = ctx.fill_text(
        &format!("\u{1f9fa} {} baskets collected", app.total_baskets),
        340.0, 200.0,
    );
    let _ = ctx.fill_text(
        &format!("\u{1f344} {} mushrooms in collection", app.collected_mushrooms.len()),
        340.0, 235.0,
    );

    // Show final collection
    let thumb_size = 40.0;
    let padding = 6.0;
    let cols = 10;
    let start_x = 80.0;
    let start_y = 270.0;

    for (i, m) in app.collected_mushrooms.iter().enumerate() {
        let col = i % cols;
        let row = i / cols;
        let mx = start_x + col as f64 * (thumb_size + padding + 20.0);
        let my = start_y + row as f64 * (thumb_size + padding + 12.0);

        if my + thumb_size > H - 60.0 {
            break;
        }

        if let Some(img) = images.get(&m.image_key) {
            let _ = ctx.draw_image_with_html_image_element_and_dw_and_dh(img, mx, my, thumb_size, thumb_size);
        } else {
            draw_fallback_sprite(ctx, &m.image_key, mx, my, thumb_size);
        }
    }

    ctx.set_fill_style_str("#f8f3e8");
    ctx.set_font("16px Georgia");
    let _ = ctx.fill_text("Press ENTER or SPACE to return to menu", 310.0, H - 20.0);
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
        "enoki" => "#FFFACD",
        "lions-mane" => "#FFF8DC",
        "matsutake" => "#D2B48C",
        "maitake" => "#696969",
        "destroying-angel" => "#FFFFFF",
        "porcini" => "#A0522D",
        "chicken-of-woods" => "#FF8C00",
        "shaggy-ink-cap" => "#F0F0F0",
        "penny-bun" => "#8B6914",
        "giant-puffball" => "#FAFAFA",
        "jelly-ear" => "#8B4513",
        "birch-polypore" => "#D3D3D3",
        "false-morel" => "#A0522D",
        "wood-ear" => "#4A3728",
        "agarikon" => "#E8DCC8",
        "jack-o-lantern" => "#FF6600",
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
