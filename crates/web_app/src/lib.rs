mod app;
mod catalog;
mod facts;
mod images;
mod input;
mod render;
mod settings;

use app::AppState;
use images::ImageCache;
use input::map_key_to_action;
use render::draw;
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

    // Preload images
    let mut image_cache = ImageCache::new();
    image_cache.preload_all();
    let image_cache = Rc::new(RefCell::new(image_cache));

    // Initial render (menu)
    draw(&context, &app.borrow(), &image_cache.borrow());

    // Keyboard input
    {
        let app = Rc::clone(&app);
        let keyboard_handler = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |event: web_sys::KeyboardEvent| {
            if let Some(action) = map_key_to_action(&event.key()) {
                event.prevent_default();
                app.borrow_mut().handle_action(action);
            }
        });
        document.add_event_listener_with_callback("keydown", keyboard_handler.as_ref().unchecked_ref())?;
        keyboard_handler.forget();
    }

    // Animation loop via requestAnimationFrame
    start_game_loop(context, app, image_cache, &window);

    Ok(())
}

fn start_game_loop(
    context: CanvasRenderingContext2d,
    app: Rc<RefCell<AppState>>,
    image_cache: Rc<RefCell<ImageCache>>,
    window: &web_sys::Window,
) {
    let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);
    let win = window.clone();
    let last_time: Rc<RefCell<f64>> = Rc::new(RefCell::new(0.0));

    *g.borrow_mut() = Some(Closure::new(move |timestamp: f64| {
        let mut lt = last_time.borrow_mut();
        let dt = if *lt == 0.0 { 0.0 } else { (timestamp - *lt) / 1000.0 };
        *lt = timestamp;

        {
            let mut state = app.borrow_mut();
            state.tick(dt.min(0.1)); // cap dt to avoid jumps
        }

        draw(&context, &app.borrow(), &image_cache.borrow());

        // Schedule next frame
        let _ = win.request_animation_frame(
            f.borrow().as_ref().unwrap().as_ref().unchecked_ref()
        );
    }));

    let _ = window.request_animation_frame(
        g.borrow().as_ref().unwrap().as_ref().unchecked_ref()
    );
}
