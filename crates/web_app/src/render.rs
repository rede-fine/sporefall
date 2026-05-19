use game_core::GameState;
use web_sys::CanvasRenderingContext2d;

pub struct RenderState<'a> {
    pub game: &'a GameState,
    pub feedback: Option<&'a str>,
}

pub fn draw(context: &CanvasRenderingContext2d, render_state: &RenderState<'_>) {
    context.set_fill_style_str("#1a2218");
    context.fill_rect(0.0, 0.0, 960.0, 540.0);

    context.set_fill_style_str("#f8f3e8");
    context.set_font("bold 48px Georgia");
    let _ = context.fill_text("Sporefall", 48.0, 74.0);

    context.set_font("24px Georgia");
    let _ = context.fill_text("Use arrow keys and space to sort the next mushroom", 48.0, 118.0);

    context.set_font("18px Georgia");
    let _ = context.fill_text(
        &format!(
            "Active lane: {} | Score: {}",
            render_state.game.active_lane() + 1,
            render_state.game.score()
        ),
        48.0,
        152.0,
    );

    if let Some(message) = render_state.feedback {
        context.set_fill_style_str("#d7c486");
        let _ = context.fill_text(message, 48.0, 182.0);
    }

    for lane_index in 0..render_state.game.lane_count() {
        let x = 72.0 + lane_index as f64 * 210.0;
        context.set_fill_style_str("#314233");
        context.fill_rect(x, 220.0, 160.0, 240.0);

        if render_state.game.settled_row()[lane_index].is_some() {
            context.set_fill_style_str("#c58b56");
            context.fill_rect(x + 18.0, 370.0, 124.0, 64.0);
        }

        if render_state.game.active_mushroom().is_some() && render_state.game.active_lane() == lane_index {
            context.set_fill_style_str("#e08d52");
            context.begin_path();
            let _ = context.arc(x + 80.0, 176.0, 34.0, 0.0, std::f64::consts::TAU);
            context.fill();
        }

        context.set_fill_style_str("#d4b375");
        let _ = context.fill_text(&format!("Lane {}", lane_index + 1), x + 34.0, 500.0);
    }
}
