use game_core::{GameConfig, GameState};

use crate::catalog::{pick_mushroom, CategoryMode};
use crate::input::InputAction;
use crate::settings::PlayerSettings;

/// Top-level game phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePhase {
    Menu,
    Playing,
}

pub struct AppState {
    pub game: GameState,
    pub settings: PlayerSettings,
    pub phase: GamePhase,
    pub category_mode: CategoryMode,
    pub menu_selection: usize,
    /// Y position of falling mushroom (0.0 = top, 1.0 = landed)
    pub fall_progress: f64,
    /// Speed: fraction of height per second
    pub fall_speed: f64,
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
            phase: GamePhase::Menu,
            category_mode: CategoryMode::Ecology,
            menu_selection: 0,
            fall_progress: 0.0,
            fall_speed: 0.4,
            feedback_message: None,
            next_mushroom_id: 0,
        }
    }

    pub fn feedback_message(&self) -> Option<&str> {
        self.feedback_message.as_deref()
    }

    pub fn start_game(&mut self) {
        self.phase = GamePhase::Playing;
        self.game = GameState::new(GameConfig {
            lane_count: self.settings.lane_count,
            points_per_clear: self.settings.points_per_clear,
            points_per_correct: self.settings.points_per_correct,
        })
        .expect("valid config");
        self.next_mushroom_id = 0;
        self.fall_progress = 0.0;
        self.feedback_message = Some("Sort the mushroom into the correct bucket!".to_owned());
        self.ensure_active_mushroom();
    }

    pub fn ensure_active_mushroom(&mut self) {
        if self.game.active_mushroom().is_some() {
            return;
        }

        let mushroom = pick_mushroom(self.next_mushroom_id, self.category_mode);

        self.game
            .spawn(mushroom, self.settings.spawn_lane.min(self.settings.lane_count - 1))
            .expect("spawn lane stays within configured bounds");
        self.next_mushroom_id += 1;
        self.fall_progress = 0.0;
    }

    /// Advance falling animation. Returns true if mushroom auto-landed.
    pub fn tick(&mut self, dt_seconds: f64) -> bool {
        if self.phase != GamePhase::Playing || self.game.active_mushroom().is_none() {
            return false;
        }
        self.fall_progress += self.fall_speed * dt_seconds;
        if self.fall_progress >= 1.0 {
            self.fall_progress = 1.0;
            self.do_drop();
            return true;
        }
        false
    }

    fn do_drop(&mut self) {
        match self.game.hard_drop() {
            Ok(feedback) => {
                let message = if feedback.row_cleared {
                    format!("Row cleared! +{} points!", feedback.awarded_points)
                } else if feedback.correct_lane {
                    format!("\u{2705} Correct! {} \u{2192} basket (+{})", feedback.mushroom_name, feedback.awarded_points)
                } else {
                    format!("\u{274c} Wrong bucket for {}", feedback.mushroom_name)
                };
                self.feedback_message = Some(message);
                self.ensure_active_mushroom();
            }
            Err(_) => {
                self.feedback_message = Some("Error placing mushroom".to_owned());
            }
        }
    }

    pub fn handle_action(&mut self, action: InputAction) {
        match self.phase {
            GamePhase::Menu => self.handle_menu_action(action),
            GamePhase::Playing => self.handle_play_action(action),
        }
    }

    fn handle_menu_action(&mut self, action: InputAction) {
        let modes = CategoryMode::all();
        match action {
            InputAction::MoveLeft | InputAction::MoveUp => {
                if self.menu_selection > 0 {
                    self.menu_selection -= 1;
                }
            }
            InputAction::MoveRight | InputAction::MoveDown => {
                if self.menu_selection + 1 < modes.len() {
                    self.menu_selection += 1;
                }
            }
            InputAction::HardDrop | InputAction::Confirm => {
                self.category_mode = modes[self.menu_selection];
                self.start_game();
            }
        }
    }

    fn handle_play_action(&mut self, action: InputAction) {
        match action {
            InputAction::MoveLeft => { let _ = self.game.move_left(); }
            InputAction::MoveRight => { let _ = self.game.move_right(); }
            InputAction::HardDrop | InputAction::Confirm => {
                self.do_drop();
            }
            _ => {}
        }
    }
}

