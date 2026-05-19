use game_core::GameState;
use web_sys::CanvasRenderingContext2d;

use crate::catalog::CATEGORY_LABELS;

pub struct RenderState<'a> {
    pub game: &'a GameState,
    pub feedback: Option<&'a str>,
}

pub fn draw(context: &CanvasRenderingContext2d, render_state: &RenderState<'_>) {
    let width = 960.0;
    let height = 540.0;

    // Background
    context.set_fill_style_str("#1a2218");
    context.fill_rect(0.0, 0.0, width, height);

    // Title
    context.set_fill_style_str("#f8f3e8");
    context.set_font("bold 36px Georgia");
    let _ = context.fill_text("Sporefall", 48.0, 56.0);

    // Instructions
    context.set_font("16px Georgia");
    context.set_fill_style_str("#a8c49a");
    let _ = context.fill_text(
        "\u{2190}\u{2192} move | SPACE drop | Sort each mushroom into the correct ecological category!",
        48.0,
        86.0,
    );

    // Active mushroom name display
    if let Some(mushroom) = render_state.game.active_mushroom() {
        context.set_fill_style_str("#f5d67a");
        context.set_font("bold 28px Georgia");
        let _ = context.fill_text(
            &format!("\u{1f344} {}", mushroom.display_name),
            48.0,
            130.0,
        );
    }

    // Score
    context.set_fill_style_str("#f8f3e8");
    context.set_font("18px Georgia");
    let _ = context.fill_text(
        &format!("Score: {}", render_state.game.score()),
        width - 160.0,
        56.0,
    );

    // Feedback
    if let Some(message) = render_state.feedback {
        context.set_fill_style_str("#d7c486");
        context.set_font("18px Georgia");
        let _ = context.fill_text(message, 48.0, 160.0);
    }

    // Lanes
    let lane_count = render_state.game.lane_count();
    let lane_width = 180.0;
    let lane_height = 280.0;
    let lane_gap = 20.0;
    let total_lanes_width = lane_count as f64 * lane_width + (lane_count as f64 - 1.0) * lane_gap;
    let lanes_start_x = (width - total_lanes_width) / 2.0;
    let lanes_start_y = 180.0;

    for lane_index in 0..lane_count {
        let x = lanes_start_x + lane_index as f64 * (lane_width + lane_gap);

        // Lane background
        context.set_fill_style_str("#2a3a2c");
        context.fill_rect(x, lanes_start_y, lane_width, lane_height);

        // Lane border
        context.set_stroke_style_str("#4a6a4c");
        context.set_line_width(2.0);
        context.stroke_rect(x, lanes_start_y, lane_width, lane_height);

        // Category label at bottom
        let label = CATEGORY_LABELS.get(lane_index).copied().unwrap_or("???");
        context.set_fill_style_str("#a8c49a");
        context.set_font("bold 14px Georgia");
        let _ = context.fill_text(label, x + 10.0, lanes_start_y + lane_height + 20.0);

        // Settled piece
        if let Some(settled) = &render_state.game.settled_row()[lane_index] {
            context.set_fill_style_str("#6b4e3d");
            context.fill_rect(x + 10.0, lanes_start_y + lane_height - 70.0, lane_width - 20.0, 60.0);
            context.set_fill_style_str("#f8f3e8");
            context.set_font("12px Georgia");
            let _ = context.fill_text(&settled.display_name, x + 16.0, lanes_start_y + lane_height - 35.0);
        }

        // Active mushroom indicator above the correct lane
        if render_state.game.active_mushroom().is_some() && render_state.game.active_lane() == lane_index {
            // Draw mushroom cap (half-circle)
            context.set_fill_style_str("#e08d52");
            context.begin_path();
            let cx = x + lane_width / 2.0;
            let cy = lanes_start_y - 30.0;
            let _ = context.arc(cx, cy, 28.0, std::f64::consts::PI, 0.0);
            context.fill();
            // Draw stem
            context.set_fill_style_str("#d4b375");
            context.fill_rect(cx - 6.0, cy, 12.0, 20.0);

            // Arrow indicator
            context.set_fill_style_str("#f5d67a");
            context.set_font("24px Georgia");
            let _ = context.fill_text("\u{25bc}", cx - 8.0, lanes_start_y - 2.0);
        }
    }
}
