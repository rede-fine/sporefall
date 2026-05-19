use game_core::{FallingMushroom, GameConfig, GameState};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("missing window"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("missing document"))?;
    let canvas = document
        .get_element_by_id("game-canvas")
        .ok_or_else(|| JsValue::from_str("missing game canvas"))?
        .dyn_into::<HtmlCanvasElement>()?;
    let context = canvas
        .get_context("2d")?
        .ok_or_else(|| JsValue::from_str("missing 2d context"))?
        .dyn_into::<CanvasRenderingContext2d>()?;

    let mut game = GameState::new(GameConfig {
        lane_count: 4,
        points_per_clear: 100,
    })
    .map_err(|error| JsValue::from_str(&format!("config error: {error:?}")))?;

    // Seed one piece so the browser shell proves the Rust game core is active.
    game.spawn(
        FallingMushroom {
            id: "amanita-muscaria".to_owned(),
            target_lane: 2,
        },
        1,
    )
    .map_err(|error| JsValue::from_str(&format!("spawn error: {error:?}")))?;

    render_placeholder(&context, &game);
    Ok(())
}

fn render_placeholder(context: &CanvasRenderingContext2d, game: &GameState) {
    context.set_fill_style_str("#1a2218");
    context.fill_rect(0.0, 0.0, 960.0, 540.0);

    context.set_fill_style_str("#f8f3e8");
    context.set_font("bold 48px Georgia");
    let _ = context.fill_text("Sporefall", 48.0, 74.0);

    context.set_font("24px Georgia");
    let _ = context.fill_text("Rust/WASM gameplay core connected", 48.0, 118.0);

    context.set_font("18px Georgia");
    let _ = context.fill_text(
        &format!("Active lane: {} | Score: {}", game.active_lane(), game.score()),
        48.0,
        152.0,
    );

    for lane_index in 0..4 {
        let x = 72.0 + lane_index as f64 * 210.0;
        context.set_fill_style_str("#314233");
        context.fill_rect(x, 220.0, 160.0, 240.0);
        context.set_fill_style_str("#d4b375");
        let _ = context.fill_text(&format!("Lane {}", lane_index + 1), x + 34.0, 500.0);
    }

    context.set_fill_style_str("#e08d52");
    context.begin_path();
    let _ = context.arc(362.0, 176.0, 34.0, 0.0, std::f64::consts::TAU);
    context.fill();
}
