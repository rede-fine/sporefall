use game_core::{FallingMushroom, GameConfig, GameState};

use crate::catalog::{pick_mushroom, CategoryMode};
use crate::facts::basket_fact;
use crate::input::InputAction;
use crate::settings::PlayerSettings;

/// Difficulty level affects what info is shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    /// Image + common name + latin name
    Normal,
    /// Image only (no names shown)
    Tricky,
    /// Emoji + latin name only
    Expert,
}

impl Difficulty {
    pub fn all() -> &'static [Difficulty] {
        &[Self::Normal, Self::Tricky, Self::Expert]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Tricky => "Tricky",
            Self::Expert => "Expert",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Normal => "Photo + Common Name + Latin Name",
            Self::Tricky => "Photo only — no names!",
            Self::Expert => "Emoji + Latin name only",
        }
    }
}

/// Top-level game phase.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GamePhase {
    Menu,
    Playing,
    Paused,
    /// Row cleared — show basket animation + fun fact
    BasketFact {
        mushrooms: Vec<FallingMushroom>,
        fact: String,
    },
    GameOver,
}

pub struct AppState {
    pub game: GameState,
    pub settings: PlayerSettings,
    pub phase: GamePhase,
    pub difficulty: Difficulty,
    pub current_level: usize, // 0-3
    pub category_mode: CategoryMode,
    pub menu_selection: usize,
    /// Y position of falling mushroom (0.0 = top, 1.0 = landed)
    pub fall_progress: f64,
    /// Speed: fraction of height per second
    pub fall_speed: f64,
    pub feedback_message: Option<String>,
    next_mushroom_id: usize,
    /// Total baskets collected across levels
    pub total_baskets: usize,
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
            difficulty: Difficulty::Normal,
            current_level: 0,
            category_mode: CategoryMode::Color,
            menu_selection: 0,
            fall_progress: 0.0,
            fall_speed: 0.35,
            feedback_message: None,
            next_mushroom_id: 0,
            total_baskets: 0,
        }
    }

    pub fn feedback_message(&self) -> Option<&str> {
        self.feedback_message.as_deref()
    }

    pub fn start_game(&mut self) {
        self.current_level = 0;
        self.category_mode = CategoryMode::for_level(0);
        self.total_baskets = 0;
        self.start_level();
    }

    fn start_level(&mut self) {
        self.phase = GamePhase::Playing;
        self.category_mode = CategoryMode::for_level(self.current_level);
        self.game = GameState::new(GameConfig {
            lane_count: self.settings.lane_count,
            points_per_clear: self.settings.points_per_clear,
            points_per_correct: self.settings.points_per_correct,
        })
        .expect("valid config");
        self.next_mushroom_id = 0;
        self.fall_progress = 0.0;
        self.feedback_message = Some(format!(
            "Level {} — Sort by {}!",
            self.current_level + 1,
            self.category_mode.display_name()
        ));
        self.ensure_active_mushroom();
    }

    pub fn advance_level(&mut self) {
        if self.current_level < 3 {
            self.current_level += 1;
            self.start_level();
        } else {
            self.phase = GamePhase::GameOver;
        }
    }

    pub fn ensure_active_mushroom(&mut self) {
        if self.game.active_mushroom().is_some() {
            return;
        }

        // Retry queue takes priority (wrong mushrooms re-enter)
        let mushroom = if let Some(retry) = self.game.pop_retry() {
            retry
        } else {
            let m = pick_mushroom(self.next_mushroom_id, self.category_mode);
            self.next_mushroom_id += 1;
            m
        };

        self.game
            .spawn(mushroom, self.settings.spawn_lane.min(self.settings.lane_count - 1))
            .expect("spawn lane stays within configured bounds");
        self.fall_progress = 0.0;
    }

    /// Advance falling animation. Returns true if mushroom auto-landed.
    pub fn tick(&mut self, dt_seconds: f64) -> bool {
        match &self.phase {
            GamePhase::Playing => {}
            _ => return false,
        }
        if self.game.active_mushroom().is_none() {
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
                if feedback.row_cleared {
                    self.total_baskets += 1;
                    let fact = basket_fact(&feedback.cleared_mushrooms);
                    self.phase = GamePhase::BasketFact {
                        mushrooms: feedback.cleared_mushrooms,
                        fact,
                    };
                    self.feedback_message = Some(format!(
                        "\u{1f9fa} Basket #{} collected! +{} points!",
                        self.total_baskets, feedback.awarded_points
                    ));
                } else if feedback.correct_lane {
                    self.feedback_message = Some(format!(
                        "\u{2705} Correct! {} \u{2192} basket (+{})",
                        feedback.mushroom_name, feedback.awarded_points
                    ));
                    self.ensure_active_mushroom();
                } else {
                    self.feedback_message = Some(format!(
                        "\u{274c} Wrong bucket for {}",
                        feedback.mushroom_name
                    ));
                    self.ensure_active_mushroom();
                }
            }
            Err(_) => {
                self.feedback_message = Some("Error placing mushroom".to_owned());
            }
        }
    }

    pub fn handle_action(&mut self, action: InputAction) {
        match &self.phase {
            GamePhase::Menu => self.handle_menu_action(action),
            GamePhase::Playing => self.handle_play_action(action),
            GamePhase::Paused => self.handle_pause_action(action),
            GamePhase::BasketFact { .. } => self.handle_basket_action(action),
            GamePhase::GameOver => self.handle_gameover_action(action),
        }
    }

    fn handle_menu_action(&mut self, action: InputAction) {
        let difficulties = Difficulty::all();
        match action {
            InputAction::MoveLeft | InputAction::MoveUp => {
                if self.menu_selection > 0 {
                    self.menu_selection -= 1;
                }
            }
            InputAction::MoveRight | InputAction::MoveDown => {
                if self.menu_selection + 1 < difficulties.len() {
                    self.menu_selection += 1;
                }
            }
            InputAction::HardDrop | InputAction::Confirm => {
                self.difficulty = difficulties[self.menu_selection];
                self.start_game();
            }
            _ => {}
        }
    }

    fn handle_play_action(&mut self, action: InputAction) {
        match action {
            InputAction::MoveLeft => { let _ = self.game.move_left(); }
            InputAction::MoveRight => { let _ = self.game.move_right(); }
            InputAction::HardDrop | InputAction::Confirm => {
                self.do_drop();
            }
            InputAction::Pause => {
                self.phase = GamePhase::Paused;
            }
            InputAction::NextLevel => {
                self.advance_level();
            }
            _ => {}
        }
    }

    fn handle_pause_action(&mut self, action: InputAction) {
        match action {
            InputAction::Pause | InputAction::Confirm => {
                self.phase = GamePhase::Playing;
            }
            InputAction::HardDrop => {
                // Quit to menu
                self.phase = GamePhase::Menu;
                self.feedback_message = None;
            }
            _ => {}
        }
    }

    fn handle_basket_action(&mut self, action: InputAction) {
        match action {
            InputAction::HardDrop | InputAction::Confirm => {
                // Resume play after viewing fact
                self.phase = GamePhase::Playing;
                self.ensure_active_mushroom();
            }
            _ => {}
        }
    }

    fn handle_gameover_action(&mut self, action: InputAction) {
        match action {
            InputAction::Confirm | InputAction::HardDrop => {
                self.phase = GamePhase::Menu;
                self.feedback_message = None;
            }
            _ => {}
        }
    }
}

