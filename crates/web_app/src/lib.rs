mod app;
mod catalog;
mod facts;
mod images;
mod inat;
mod input;
mod leaderboard;
mod render;
mod settings;
mod ui;

use app::AppState;
use images::ImageCache;
use inat::{import_catalog, ImportRequest};
use input::map_key_to_action;
use render::draw;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{
    CanvasRenderingContext2d, Document, HtmlButtonElement, HtmlCanvasElement, HtmlElement,
    HtmlInputElement, PointerEvent,
};

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
    apply_viewport(&canvas, &app, &window);

    let mut image_cache = ImageCache::new();
    image_cache.preload_all();
    let image_cache = Rc::new(RefCell::new(image_cache));

    wire_inat_controls(&document, Rc::clone(&app), Rc::clone(&image_cache))?;

    draw(&context, &app.borrow(), &image_cache.borrow());

    {
        let app = Rc::clone(&app);
        let doc_for_closure = document.clone();
        let keyboard_handler =
            Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |event: web_sys::KeyboardEvent| {
                if !should_capture_game_input(&doc_for_closure) {
                    return;
                }
                if let Some(action) = map_key_to_action(&event.key()) {
                    event.prevent_default();
                    app.borrow_mut().handle_action(action);
                }
            });
        document.add_event_listener_with_callback(
            "keydown",
            keyboard_handler.as_ref().unchecked_ref(),
        )?;
        keyboard_handler.forget();
    }

    {
        let app = Rc::clone(&app);
        let canvas_for_closure = canvas.clone();
        let pointer_handler = Closure::<dyn FnMut(PointerEvent)>::new(move |event: PointerEvent| {
            if let Some((x, y)) = pointer_position(&canvas_for_closure, &event) {
                event.prevent_default();
                app.borrow_mut().handle_pointer(x, y);
            }
        });
        canvas.add_event_listener_with_callback(
            "pointerdown",
            pointer_handler.as_ref().unchecked_ref(),
        )?;
        pointer_handler.forget();
    }

    {
        let app = Rc::clone(&app);
        let canvas_for_closure = canvas.clone();
        let context = context.clone();
        let image_cache = Rc::clone(&image_cache);
        let window_for_closure = window.clone();
        let resize_handler = Closure::<dyn FnMut()>::new(move || {
            apply_viewport(&canvas_for_closure, &app, &window_for_closure);
            draw(&context, &app.borrow(), &image_cache.borrow());
        });
        window.add_event_listener_with_callback("resize", resize_handler.as_ref().unchecked_ref())?;
        resize_handler.forget();
    }

    let leaderboard = leaderboard::LeaderboardController::bootstrap(
        Rc::clone(&app),
        document.clone(),
    )
    .ok();

    start_game_loop(context, app, image_cache, leaderboard, &window);

    Ok(())
}

fn start_game_loop(
    context: CanvasRenderingContext2d,
    app: Rc<RefCell<AppState>>,
    image_cache: Rc<RefCell<ImageCache>>,
    leaderboard: Option<Rc<RefCell<leaderboard::LeaderboardController>>>,
    window: &web_sys::Window,
) {
    let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);
    let win = window.clone();
    let last_time: Rc<RefCell<f64>> = Rc::new(RefCell::new(0.0));

    *g.borrow_mut() = Some(Closure::new(move |timestamp: f64| {
        let mut last_timestamp = last_time.borrow_mut();
        let dt = if *last_timestamp == 0.0 {
            0.0
        } else {
            (timestamp - *last_timestamp) / 1000.0
        };
        *last_timestamp = timestamp;

        {
            let mut state = app.borrow_mut();
            state.tick(dt.min(0.1));
        }

        if let Some(ref lb) = leaderboard {
            leaderboard::LeaderboardController::tick(lb);
        }

        draw(&context, &app.borrow(), &image_cache.borrow());

        let _ = win.request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref());
    }));

    let _ = window.request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref());
}

fn wire_inat_controls(
    document: &Document,
    app: Rc<RefCell<AppState>>,
    image_cache: Rc<RefCell<ImageCache>>,
) -> Result<(), JsValue> {
    let user_input = get_input(document, "inat-user")?;
    let start_input = get_input(document, "inat-start-date")?;
    let end_input = get_input(document, "inat-end-date")?;
    let button = get_button(document, "inat-import-button")?;
    let status = get_html(document, "inat-status")?;

    let (default_start, default_end) = default_date_range();
    start_input.set_value(&default_start);
    end_input.set_value(&default_end);
    set_status(
        &status,
        "idle",
        "Pick a username and date range, then import fungi observations.",
    );

    let user_input_for_click = user_input.clone();
    let start_input_for_click = start_input.clone();
    let end_input_for_click = end_input.clone();
    let button_for_click = button.clone();
    let status_for_click = status.clone();
    let click_handler = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        event.prevent_default();
        let request = ImportRequest {
            user_login: user_input_for_click.value(),
            start_date: start_input_for_click.value(),
            end_date: end_input_for_click.value(),
        };

        if let Err(error) = request.validate() {
            set_status(&status_for_click, "error", &error);
            app.borrow_mut().feedback_message = Some(error);
            return;
        }

        button_for_click.set_disabled(true);
        set_status(
            &status_for_click,
            "loading",
            "Importing fungi observations from iNaturalist...",
        );

        let app = Rc::clone(&app);
        let image_cache = Rc::clone(&image_cache);
        let status = status_for_click.clone();
        let button = button_for_click.clone();

        spawn_local(async move {
            match import_catalog(request).await {
                Ok(imported) => {
                    let summary = imported.summary.clone();
                    for entry in &imported.entries {
                        if let Some(image_url) = &entry.provenance.image_url {
                            image_cache
                                .borrow_mut()
                                .preload_dynamic(&entry.image_key, image_url);
                        }
                    }

                    let matched_count = summary.matched_species_count;
                    let user_login = summary.user_login.clone();
                    let summary_message = format!(
                        "Imported {} playable species for @{} and started a personalized run.",
                        matched_count, user_login
                    );

                    let mut state = app.borrow_mut();
                    state.remember_import(imported.entries, summary);
                    match state.start_imported_game() {
                        Ok(()) => {
                            state.feedback_message = Some(summary_message.clone());
                            set_status(&status, "success", &summary_message);
                        }
                        Err(error) => {
                            state.feedback_message = Some(error.clone());
                            set_status(&status, "error", &error);
                        }
                    }
                }
                Err(error) => {
                    app.borrow_mut().feedback_message = Some(error.clone());
                    set_status(&status, "error", &error);
                }
            }

            button.set_disabled(false);
        });
    });

    button.add_event_listener_with_callback("click", click_handler.as_ref().unchecked_ref())?;
    click_handler.forget();

    Ok(())
}

fn should_capture_game_input(document: &Document) -> bool {
    let Some(active_element) = document.active_element() else {
        return true;
    };

    !matches!(
        active_element.tag_name().as_str(),
        "INPUT" | "BUTTON" | "TEXTAREA" | "SELECT"
    )
}

fn default_date_range() -> (String, String) {
    let end = js_sys::Date::new_0();
    let start = js_sys::Date::new_0();
    start.set_time(end.get_time() - 365.0 * 24.0 * 60.0 * 60.0 * 1000.0);
    (format_date_input(&start), format_date_input(&end))
}

fn format_date_input(date: &js_sys::Date) -> String {
    format!(
        "{:04}-{:02}-{:02}",
        date.get_full_year() as i32,
        date.get_month() as u32 + 1,
        date.get_date() as u32
    )
}

fn set_status(status: &HtmlElement, state: &str, text: &str) {
    status.set_class_name(&format!("inat-status inat-status--{state}"));
    status.set_inner_text(text);
}

fn get_input(document: &Document, id: &str) -> Result<HtmlInputElement, JsValue> {
    document
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str(&format!("missing element #{id}")))?
        .dyn_into::<HtmlInputElement>()
        .map_err(Into::into)
}

fn get_button(document: &Document, id: &str) -> Result<HtmlButtonElement, JsValue> {
    document
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str(&format!("missing element #{id}")))?
        .dyn_into::<HtmlButtonElement>()
        .map_err(Into::into)
}

fn get_html(document: &Document, id: &str) -> Result<HtmlElement, JsValue> {
    document
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str(&format!("missing element #{id}")))?
        .dyn_into::<HtmlElement>()
        .map_err(Into::into)
}

fn apply_viewport(
    canvas: &HtmlCanvasElement,
    app: &Rc<RefCell<AppState>>,
    window: &web_sys::Window,
) {
    let inner_width = window
        .inner_width()
        .ok()
        .and_then(|value| value.as_f64())
        .unwrap_or(1024.0);
    let inner_height = window
        .inner_height()
        .ok()
        .and_then(|value| value.as_f64())
        .unwrap_or(768.0);
    let viewport = ui::Viewport::choose(inner_width, inner_height);
    canvas.set_width(viewport.width as u32);
    canvas.set_height(viewport.height as u32);
    app.borrow_mut().set_viewport(viewport);
}

fn pointer_position(canvas: &HtmlCanvasElement, event: &PointerEvent) -> Option<(f64, f64)> {
    let rect = canvas.get_bounding_client_rect();
    if rect.width() <= 0.0 || rect.height() <= 0.0 {
        return None;
    }

    let x = (f64::from(event.client_x()) - rect.left()) * f64::from(canvas.width()) / rect.width();
    let y =
        (f64::from(event.client_y()) - rect.top()) * f64::from(canvas.height()) / rect.height();
    Some((x, y))
}
