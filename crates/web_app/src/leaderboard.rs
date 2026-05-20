use std::{cell::RefCell, rc::Rc};

use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Document, Element, HtmlInputElement, KeyboardEvent};

use crate::app::{AppState, GamePhase};

const LEADERBOARD_API_BASE: &str = "http://127.0.0.1:8787";
const MAX_ENTRIES: usize = 10;
const MAX_NAME_LENGTH: usize = 24;

#[derive(Debug, Clone, Deserialize)]
struct LeaderboardEntry {
    id: i64,
    player_name: String,
    score: u32,
    recorded_at: String,
}

#[derive(Debug, Clone, Deserialize)]
struct LeaderboardResponse {
    entries: Vec<LeaderboardEntry>,
}

#[derive(Debug, Clone, Deserialize)]
struct SubmissionResponse {
    accepted: bool,
    rank: Option<usize>,
    entries: Vec<LeaderboardEntry>,
}

pub struct LeaderboardController {
    app: Rc<RefCell<AppState>>,
    status_el: Element,
    list_el: Element,
    modal_el: Element,
    modal_score_el: Element,
    modal_status_el: Element,
    input_el: HtmlInputElement,
    has_loaded: bool,
    fetch_in_flight: bool,
    submit_in_flight: bool,
    prompt_open: bool,
    prompt_score: Option<u32>,
    evaluated_score: Option<u32>,
    entries: Vec<LeaderboardEntry>,
}

enum TickAction {
    Refresh,
    OpenPrompt(u32),
}

impl LeaderboardController {
    pub fn bootstrap(app: Rc<RefCell<AppState>>, document: Document) -> Result<Rc<RefCell<Self>>, JsValue> {
        let input_el = document
            .get_element_by_id("leaderboard-name-input")
            .ok_or_else(|| JsValue::from_str("missing leaderboard name input"))?
            .dyn_into::<HtmlInputElement>()?;

        let controller = Rc::new(RefCell::new(Self {
            app,
            status_el: get_element(&document, "leaderboard-status")?,
            list_el: get_element(&document, "leaderboard-list")?,
            modal_el: get_element(&document, "leaderboard-modal")?,
            modal_score_el: get_element(&document, "leaderboard-modal-score")?,
            modal_status_el: get_element(&document, "leaderboard-modal-status")?,
            input_el,
            has_loaded: false,
            fetch_in_flight: false,
            submit_in_flight: false,
            prompt_open: false,
            prompt_score: None,
            evaluated_score: None,
            entries: Vec::new(),
        }));

        {
            let controller_ref = controller.borrow();
            controller_ref.render_entries();
            controller_ref.set_status("Loading the persistent Top 10 leaderboard...");
        }

        attach_click_handler(&document, "leaderboard-submit", {
            let controller = Rc::clone(&controller);
            move || LeaderboardController::submit_prompt(&controller)
        })?;

        attach_click_handler(&document, "leaderboard-cancel", {
            let controller = Rc::clone(&controller);
            move || LeaderboardController::cancel_prompt(&controller)
        })?;

        {
            let controller_for_closure = Rc::clone(&controller);
            let input_handler = Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
                match event.key().as_str() {
                    "Enter" => {
                        event.prevent_default();
                        LeaderboardController::submit_prompt(&controller_for_closure);
                    }
                    "Escape" => {
                        event.prevent_default();
                        LeaderboardController::cancel_prompt(&controller_for_closure);
                    }
                    _ => {}
                }
            });
            controller.borrow().input_el.add_event_listener_with_callback(
                "keydown",
                input_handler.as_ref().unchecked_ref(),
            )?;
            input_handler.forget();
        }

        Self::refresh(&controller);
        Ok(controller)
    }

    pub fn is_prompt_open(&self) -> bool {
        self.prompt_open
    }

    pub fn tick(controller: &Rc<RefCell<Self>>) {
        let (is_game_over, score) = {
            let controller_ref = controller.borrow();
            let app = controller_ref.app.borrow();
            (matches!(app.phase, GamePhase::GameOver), app.game.score())
        };

        let action = {
            let mut controller_ref = controller.borrow_mut();

            if !is_game_over {
                controller_ref.evaluated_score = None;
                if controller_ref.prompt_open {
                    controller_ref.hide_prompt();
                }
                if !controller_ref.has_loaded && !controller_ref.fetch_in_flight {
                    Some(TickAction::Refresh)
                } else {
                    None
                }
            } else if !controller_ref.has_loaded {
                if !controller_ref.fetch_in_flight {
                    Some(TickAction::Refresh)
                } else {
                    None
                }
            } else if controller_ref.fetch_in_flight
                || controller_ref.submit_in_flight
                || controller_ref.prompt_open
                || controller_ref.evaluated_score == Some(score)
            {
                None
            } else if controller_ref.qualifies(score) {
                Some(TickAction::OpenPrompt(score))
            } else {
                controller_ref.evaluated_score = Some(score);
                controller_ref.set_status("That run did not reach the current Top 10.");
                None
            }
        };

        match action {
            Some(TickAction::Refresh) => Self::refresh(controller),
            Some(TickAction::OpenPrompt(score)) => controller.borrow_mut().open_prompt(score),
            None => {}
        }
    }

    fn refresh(controller: &Rc<RefCell<Self>>) {
        {
            let mut controller_ref = controller.borrow_mut();
            controller_ref.fetch_in_flight = true;
            controller_ref.set_status("Refreshing leaderboard from the local SQLite service...");
        }

        let controller = Rc::clone(controller);
        spawn_local(async move {
            match Request::get(&format!("{LEADERBOARD_API_BASE}/api/leaderboard"))
                .send()
                .await
            {
                Ok(response) => {
                    if !response.ok() {
                        Self::handle_fetch_error(
                            &controller,
                            &format!("Leaderboard request failed with status {}.", response.status()),
                        );
                        return;
                    }
                    match response.json::<LeaderboardResponse>().await {
                        Ok(data) => Self::handle_fetch_success(&controller, data.entries),
                        Err(err) => Self::handle_fetch_error(
                            &controller,
                            &format!("Could not parse leaderboard response: {err}"),
                        ),
                    }
                }
                Err(_) => {
                    Self::handle_fetch_error(
                        &controller,
                        "Leaderboard service is offline. Start `python leaderboard_service.py`.",
                    );
                }
            }
        });
    }

    fn handle_fetch_success(controller: &Rc<RefCell<Self>>, entries: Vec<LeaderboardEntry>) {
        let mut controller_ref = controller.borrow_mut();
        controller_ref.fetch_in_flight = false;
        controller_ref.has_loaded = true;
        controller_ref.entries = entries;
        controller_ref.render_entries();

        if controller_ref.entries.is_empty() {
            controller_ref.set_status("No scores yet. The next completed run can claim #1.");
        } else {
            controller_ref.set_status("Persistent Top 10 loaded from the local SQLite leaderboard.");
        }
    }

    fn handle_fetch_error(controller: &Rc<RefCell<Self>>, message: &str) {
        let mut controller_ref = controller.borrow_mut();
        controller_ref.fetch_in_flight = false;
        controller_ref.set_status(message);
        if !controller_ref.has_loaded {
            controller_ref.list_el.set_inner_html(
                "<li class=\"leaderboard-empty\">Leaderboard unavailable. Start <code>python leaderboard_service.py</code> to persist scores.</li>",
            );
        }
    }

    fn qualifies(&self, score: u32) -> bool {
        self.entries.len() < MAX_ENTRIES
            || self
                .entries
                .last()
                .map(|entry| score >= entry.score)
                .unwrap_or(true)
    }

    fn open_prompt(&mut self, score: u32) {
        self.prompt_open = true;
        self.prompt_score = Some(score);
        self.input_el.set_value("");
        self.set_hidden(&self.modal_el, false);
        self.modal_score_el
            .set_text_content(Some(&format!("Your {} point run reached the Top 10. Enter a player name to save it.", score)));
        self.modal_status_el
            .set_text_content(Some("Player names are capped at 24 characters."));
        let _ = self.input_el.focus();
    }

    fn hide_prompt(&mut self) {
        self.prompt_open = false;
        self.prompt_score = None;
        self.submit_in_flight = false;
        self.input_el.set_value("");
        self.set_hidden(&self.modal_el, true);
    }

    fn cancel_prompt(controller: &Rc<RefCell<Self>>) {
        let mut controller_ref = controller.borrow_mut();
        controller_ref.evaluated_score = controller_ref.prompt_score;
        controller_ref.hide_prompt();
        controller_ref.set_status("Skipped saving that score.");
    }

    fn submit_prompt(controller: &Rc<RefCell<Self>>) {
        let (player_name, score) = {
            let mut controller_ref = controller.borrow_mut();
            let Some(score) = controller_ref.prompt_score else {
                return;
            };

            let player_name = controller_ref.input_el.value().trim().to_owned();
            if player_name.is_empty() {
                controller_ref
                    .modal_status_el
                    .set_text_content(Some("Enter a player name before saving."));
                return;
            }

            if player_name.chars().count() > MAX_NAME_LENGTH {
                controller_ref.modal_status_el.set_text_content(Some(
                    "That name is too long for the leaderboard. Keep it under 24 characters.",
                ));
                return;
            }

            controller_ref.submit_in_flight = true;
            controller_ref
                .modal_status_el
                .set_text_content(Some("Saving your score to the leaderboard..."));
            (player_name, score)
        };

        let controller = Rc::clone(controller);
        let name_for_status = player_name.clone();
        spawn_local(async move {
            let body = format!(
                r#"{{"player_name":"{}","score":{}}}"#,
                player_name.replace('\\', "\\\\").replace('"', "\\\""),
                score
            );

            let result = Request::post(&format!("{LEADERBOARD_API_BASE}/api/leaderboard"))
                .header("Content-Type", "application/json")
                .body(body)
                .unwrap()
                .send()
                .await;

            match result {
                Ok(response) => {
                    if !response.ok() {
                        Self::handle_submit_error(
                            &controller,
                            &format!("Save failed with status {}.", response.status()),
                        );
                        return;
                    }
                    match response.json::<SubmissionResponse>().await {
                        Ok(data) => {
                            let mut controller_ref = controller.borrow_mut();
                            controller_ref.submit_in_flight = false;
                            controller_ref.evaluated_score = Some(score);
                            controller_ref.entries = data.entries;
                            controller_ref.has_loaded = true;
                            controller_ref.render_entries();
                            controller_ref.hide_prompt();

                            if data.accepted {
                                controller_ref.set_status(&format!(
                                    "{} entered the Top 10{}.",
                                    name_for_status,
                                    data.rank
                                        .map(|rank| format!(" at #{}", rank))
                                        .unwrap_or_default()
                                ));
                            } else {
                                controller_ref.set_status(
                                    "Score saved, but it did not remain in the Top 10 after ranking.",
                                );
                            }
                        }
                        Err(err) => Self::handle_submit_error(
                            &controller,
                            &format!("Save response could not be decoded: {err}"),
                        ),
                    }
                }
                Err(_) => {
                    Self::handle_submit_error(
                        &controller,
                        "Could not reach the leaderboard service. Make sure it is running.",
                    );
                }
            }
        });
    }

    fn handle_submit_error(controller: &Rc<RefCell<Self>>, message: &str) {
        let mut controller_ref = controller.borrow_mut();
        controller_ref.submit_in_flight = false;
        controller_ref.modal_status_el.set_text_content(Some(message));
    }

    fn render_entries(&self) {
        if self.entries.is_empty() {
            self.list_el.set_inner_html(
                "<li class=\"leaderboard-empty\">No leaderboard entries yet. Finish a run to seed the first score.</li>",
            );
            return;
        }

        let mut html = String::new();
        for (index, entry) in self.entries.iter().enumerate() {
            let timestamp = escape_html(&format_timestamp(&entry.recorded_at));
            let player_name = escape_html(&entry.player_name);
            let accent = if index == 0 { " leaderboard-item--top" } else { "" };
            html.push_str(&format!(
                "<li class=\"leaderboard-item{accent}\">\
                    <span class=\"leaderboard-rank\">#{rank}</span>\
                    <div class=\"leaderboard-meta\">\
                        <span class=\"leaderboard-name\">{player_name}</span>\
                        <span class=\"leaderboard-date\">{timestamp}</span>\
                    </div>\
                    <span class=\"leaderboard-score\">{score}</span>\
                </li>",
                accent = accent,
                rank = index + 1,
                player_name = player_name,
                timestamp = timestamp,
                score = entry.score,
            ));
        }

        self.list_el.set_inner_html(&html);
    }

    fn set_status(&self, message: &str) {
        self.status_el.set_text_content(Some(message));
    }

    fn set_hidden(&self, element: &Element, hidden: bool) {
        if hidden {
            let _ = element.set_attribute("hidden", "");
        } else {
            let _ = element.remove_attribute("hidden");
        }
    }
}

fn get_element(document: &Document, id: &str) -> Result<Element, JsValue> {
    document
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str(&format!("missing element `{}`", id)))
}

fn attach_click_handler<F>(document: &Document, id: &str, callback: F) -> Result<(), JsValue>
where
    F: FnMut() + 'static,
{
    let element = get_element(document, id)?;
    let handler = Closure::<dyn FnMut()>::new(callback);
    element.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())?;
    handler.forget();
    Ok(())
}

fn format_timestamp(recorded_at: &str) -> String {
    if let Some((date, time)) = recorded_at.split_once('T') {
        let time = time
            .trim_end_matches('Z')
            .split('+')
            .next()
            .unwrap_or(time)
            .chars()
            .take(5)
            .collect::<String>();
        if !time.is_empty() {
            return format!("{} {} UTC", date, time);
        }
    }

    recorded_at.to_owned()
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
