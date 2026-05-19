use game_core::{GameConfig, GameState};

use crate::catalog::pick_mushroom;
use crate::input::InputAction;
use crate::settings::PlayerSettings;

pub struct AppState {
    pub game: GameState,
    pub settings: PlayerSettings,
    feedback_message: Option<String>,
    next_mushroom_id: usize,
}

impl AppState {
    pub fn new(settings: PlayerSettings) -> Self {
        let game = GameState::new(GameConfig {
            lane_count: settings.lane_count,
            points_per_clear: settings.points_per_clear,
            points_per_correct: settings.points_per_correct,
        })
        .expect("settings create a valid game configuration");

        Self {
            game,
            settings,
            feedback_message: None,
            next_mushroom_id: 0,
        }
    }

    pub fn feedback_message(&self) -> Option<&str> {
        self.feedback_message.as_deref()
    }

    pub fn ensure_active_mushroom(&mut self) {
        if self.game.active_mushroom().is_some() {
            return;
        }

        let mushroom = pick_mushroom(self.next_mushroom_id);

        self.game
            .spawn(mushroom, self.settings.spawn_lane.min(self.settings.lane_count - 1))
            .expect("spawn lane stays within configured bounds");
        self.next_mushroom_id += 1;
    }

    pub fn handle_action(&mut self, action: InputAction) {
        let result = match action {
            InputAction::MoveLeft => self.game.move_left().map(|_| None),
            InputAction::MoveRight => self.game.move_right().map(|_| None),
            InputAction::HardDrop => self.game.hard_drop().map(|feedback| {
                let message = if feedback.row_cleared {
                    format!("Row cleared for {} points", feedback.awarded_points)
                } else if feedback.correct_lane {
                    "Correct bucket".to_owned()
                } else {
                    "Wrong bucket".to_owned()
                };
                Some(message)
            }),
        };

        match result {
            Ok(message) => {
                if let Some(message) = message {
                    self.feedback_message = Some(message);
                    self.ensure_active_mushroom();
                }
            }
            Err(_) => {
                self.feedback_message = Some("Action unavailable".to_owned());
            }
        }
    }
}

