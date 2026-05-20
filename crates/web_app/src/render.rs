use web_sys::CanvasRenderingContext2d;

use crate::app::{AppState, GamePhase};
use crate::catalog::CategoryMode;

const W: f64 = 960.0;
const H: f64 = 540.0;

pub fn draw(ctx: &CanvasRenderingContext2d, app: &AppState) {
    ctx.set_fill_style_str("#1a2218");
    ctx.fill_rect(0.0, 0.0, W, H);

    match app.phase {
        GamePhase::Menu => draw_menu(ctx, app),
        GamePhase::Playing => draw_playing(ctx, app),
    }
}

fn draw_menu(ctx: &CanvasRenderingContext2d, app: &AppState) {
    // Title
    ctx.set_fill_style_str("#f8f3e8");
    ctx.set_font("bold 48px Georgia");
    let _ = ctx.fill_text("\u{1f344} Sporefall", 260.0, 80.0);

    ctx.set_font("20px Georgia");
    ctx.set_fill_style_str("#a8c49a");
    let _ = ctx.fill_text("Choose a category to sort mushrooms by:", 260.0, 130.0);

    // Category options
    let modes = CategoryMode::all();
    for (i, mode) in modes.iter().enumerate() {
        let y = 180.0 + i as f64 * 80.0;
        let selected = i == app.menu_selection;

        if selected {
            ctx.set_fill_style_str("#3a5a3c");
            ctx.fill_rect(220.0, y - 10.0, 520.0, 65.0);
            ctx.set_fill_style_str("#f5d67a");
            ctx.set_font("bold 14px Georgia");
            let _ = ctx.fill_text("\u{25b6}", 230.0, y + 28.0);
        }

        ctx.set_fill_style_str(if selected { "#f5d67a" } else { "#f8f3e8" });
        ctx.set_font("bold 24px Georgia");
        let _ = ctx.fill_text(mode.display_name(), 260.0, y + 28.0);

        ctx.set_fill_style_str("#8aaa7c");
        ctx.set_font("14px Georgia");
        let labels = mode.labels().join("  |  ");
        let _ = ctx.fill_text(&labels, 260.0, y + 48.0);
    }

    // Instructions
    ctx.set_fill_style_str("#8aaa7c");
    ctx.set_font("16px Georgia");
    let _ = ctx.fill_text("\u{2191}\u{2193} select   ENTER/SPACE start", 320.0, 510.0);
}

fn draw_playing(ctx: &CanvasRenderingContext2d, app: &AppState) {
    let lane_count = app.game.lane_count();
    let lane_width = 180.0;
    let lane_gap = 16.0;
    let total_w = lane_count as f64 * lane_width + (lane_count as f64 - 1.0) * lane_gap;
    let lanes_x = (W - total_w) / 2.0;
    let lanes_top = 160.0;
    let lanes_height = 280.0;
    let lanes_bottom = lanes_top + lanes_height;

    // Header: score + category
    ctx.set_fill_style_str("#f8f3e8");
    ctx.set_font("bold 20px Georgia");
    let _ = ctx.fill_text(
        &format!("\u{1f344} Sporefall  \u{2014}  {}", app.category_mode.display_name()),
        30.0, 36.0,
    );

    ctx.set_fill_style_str("#f5d67a");
    ctx.set_font("bold 20px Georgia");
    let _ = ctx.fill_text(&format!("Score: {}", app.game.score()), W - 160.0, 36.0);

    // Basket display (top-right area)
    let basket = app.game.basket();
    if !basket.is_empty() {
        ctx.set_fill_style_str("#8aaa7c");
        ctx.set_font("14px Georgia");
        let _ = ctx.fill_text(&format!("\u{1f9fa} Basket: {}", basket.len()), W - 160.0, 60.0);
    }

    // Feedback message
    if let Some(msg) = app.feedback_message() {
        ctx.set_fill_style_str("#d7c486");
        ctx.set_font("18px Georgia");
        let _ = ctx.fill_text(msg, 30.0, 64.0);
    }

    // Active mushroom info at top
    if let Some(mushroom) = app.game.active_mushroom() {
        ctx.set_fill_style_str("#f5d67a");
        ctx.set_font("bold 26px Georgia");
        let _ = ctx.fill_text(&mushroom.display_name, 30.0, 110.0);

        // Draw mushroom image placeholder
        draw_mushroom_sprite(ctx, &mushroom.image_key, 30.0, 120.0, 48.0);
    }

    // Lane backgrounds and labels
    let labels = app.category_mode.labels();
    for i in 0..lane_count {
        let x = lanes_x + i as f64 * (lane_width + lane_gap);

        // Lane background
        ctx.set_fill_style_str("#2a3a2c");
        ctx.fill_rect(x, lanes_top, lane_width, lanes_height);
        ctx.set_stroke_style_str("#4a6a4c");
        ctx.set_line_width(2.0);
        ctx.stroke_rect(x, lanes_top, lane_width, lanes_height);

        // Bucket label at bottom
        let label = labels.get(i).copied().unwrap_or("???");
        ctx.set_fill_style_str("#a8c49a");
        ctx.set_font("bold 13px Georgia");
        let _ = ctx.fill_text(label, x + 8.0, lanes_bottom + 18.0);

        // Draw stacked mushrooms in lane
        let lane_stack = &app.game.lanes()[i];
        for (j, m) in lane_stack.iter().enumerate() {
            let piece_h = 50.0;
            let piece_y = lanes_bottom - (j as f64 + 1.0) * piece_h;
            if piece_y < lanes_top {
                break;
            }
            ctx.set_fill_style_str("#5a3a2a");
            ctx.fill_rect(x + 8.0, piece_y, lane_width - 16.0, piece_h - 4.0);
            draw_mushroom_sprite(ctx, &m.image_key, x + 12.0, piece_y + 4.0, 36.0);
            ctx.set_fill_style_str("#f8f3e8");
            ctx.set_font("11px Georgia");
            let _ = ctx.fill_text(&m.display_name, x + 52.0, piece_y + 28.0);
        }
    }

    // Falling mushroom (animated position)
    if app.game.active_mushroom().is_some() {
        let active_lane = app.game.active_lane();
        let x = lanes_x + active_lane as f64 * (lane_width + lane_gap);
        let fall_y = lanes_top * app.fall_progress + 90.0 * (1.0 - app.fall_progress);

        // Active indicator (mushroom shape falling)
        ctx.set_fill_style_str("#e08d52");
        ctx.begin_path();
        let cx = x + lane_width / 2.0;
        let _ = ctx.arc(cx, fall_y, 24.0, std::f64::consts::PI, 0.0);
        ctx.fill();
        ctx.set_fill_style_str("#d4b375");
        ctx.fill_rect(cx - 5.0, fall_y, 10.0, 16.0);

        // Arrow showing where it will land
        ctx.set_fill_style_str("#f5d67a");
        ctx.set_font("20px Georgia");
        let _ = ctx.fill_text("\u{25bc}", cx - 7.0, lanes_top + 16.0);

        // Lane highlight
        ctx.set_stroke_style_str("#f5d67a");
        ctx.set_line_width(3.0);
        ctx.stroke_rect(x, lanes_top, lane_width, lanes_height);
    }

    // Instructions bar
    ctx.set_fill_style_str("#5a7a5c");
    ctx.set_font("14px Georgia");
    let _ = ctx.fill_text(
        "\u{2190}\u{2192} move   SPACE drop   Sort mushrooms into the correct bucket!",
        lanes_x, H - 16.0,
    );
}

/// Draw a simple colored mushroom sprite based on image_key.
fn draw_mushroom_sprite(ctx: &CanvasRenderingContext2d, image_key: &str, x: f64, y: f64, size: f64) {
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

    // Cap
    ctx.set_fill_style_str(cap_color);
    ctx.begin_path();
    let _ = ctx.arc(cx, cy, r * 0.7, std::f64::consts::PI, 0.0);
    ctx.fill();

    // Stem
    ctx.set_fill_style_str("#F5F5DC");
    ctx.fill_rect(cx - r * 0.15, cy, r * 0.3, r * 0.7);
}
