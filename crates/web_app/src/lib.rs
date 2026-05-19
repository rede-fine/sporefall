mod app;
mod catalog;
mod input;
mod render;
mod settings;

use app::AppState;
use input::map_key_to_action;
use render::{draw, RenderState};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
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

    let app = Rc::new(RefCell::new(AppState::new(Default::default())));
    app.borrow_mut().ensure_active_mushroom();

    sync_overlay(&document, &app.borrow());
    render_app(&context, &app.borrow());
    bind_keyboard_events(&document, &context, &app)?;
    Ok(())
}

fn render_app(context: &CanvasRenderingContext2d, app: &AppState) {
    draw(
        context,
        &RenderState {
            game: &app.game,
            feedback: app.feedback_message(),
        },
    );
}

fn bind_keyboard_events(
    document: &web_sys::Document,
    context: &CanvasRenderingContext2d,
    app: &Rc<RefCell<AppState>>,
) -> Result<(), JsValue> {
    let context = Rc::new(context.clone());
    let document = document.clone();
    let app = Rc::clone(app);
    let doc_for_closure = document.clone();
    let keyboard_handler = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |event: web_sys::KeyboardEvent| {
        if let Some(action) = map_key_to_action(&event.key()) {
            event.prevent_default();
            let mut app = app.borrow_mut();
            app.handle_action(action);
            sync_overlay(&doc_for_closure, &app);
            render_app(&context, &app);
        }
    });

    document.add_event_listener_with_callback("keydown", keyboard_handler.as_ref().unchecked_ref())?;
    keyboard_handler.forget();
    Ok(())
}

fn sync_overlay(document: &web_sys::Document, app: &AppState) {
    if let Some(score_node) = document.get_element_by_id("score-value") {
        score_node.set_text_content(Some(&app.game.score().to_string()));
    }

    if let Some(feedback_node) = document.get_element_by_id("feedback-value") {
        feedback_node.set_text_content(Some(app.feedback_message().unwrap_or("Move with arrow keys, then press space to drop.")));
    }
}
