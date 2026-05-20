use game_core::{FallingMushroom, GameConfig, GameState};
use std::collections::HashSet;

use crate::catalog::{
    built_in_catalog, cap_catalog, pick_mushroom, CategoryMode, RuntimeCatalogEntry, Variety,
};
use crate::facts::basket_fact;
use crate::inat::ImportSummary;
use crate::input::InputAction;
use crate::settings::PlayerSettings;
use crate::ui::{self, Viewport};

/// Difficulty level affects what info is shown (game mode).
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
            Self::Tricky => "Photo only - no names!",
            Self::Expert => "Emoji + Latin name only",
        }
    }
}

/// Center screen animation effect.
#[derive(Debug, Clone)]
pub enum CenterAnimation {
    /// Green checkmark that pulses and fades
    Correct { progress: f64 },
    /// Red X that grows then fades
    Wrong { progress: f64 },
    /// Basket emoji that bounces before the fact screen
    BasketCollected { progress: f64 },
}

impl CenterAnimation {
    pub fn is_done(&self) -> bool {
        match self {
            Self::Correct { progress } => *progress >= 1.0,
            Self::Wrong { progress } => *progress >= 1.0,
            Self::BasketCollected { progress } => *progress >= 1.0,
        }
    }

    pub fn advance(&mut self, dt: f64) {
        let speed = 2.0; // full animation in 0.5s
        match self {
            Self::Correct { progress } => *progress += dt * speed,
            Self::Wrong { progress } => *progress += dt * speed,
            Self::BasketCollected { progress } => *progress += dt * 1.5,
        }
    }
}

/// Top-level game phase.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GamePhase {
    Menu,
    Playing,
    Paused,
    /// Row cleared - show basket animation + fun fact
    BasketFact {
        mushrooms: Vec<FallingMushroom>,
        fact: String,
    },
    /// All mushrooms in this level sorted - review basket then proceed
    LevelComplete,
    GameOver,
}

pub struct AppState {
    pub game: GameState,
    pub settings: PlayerSettings,
    pub phase: GamePhase,
    pub difficulty: Difficulty,
    pub variety: Variety,
    pub current_level: usize, // 0-3
    pub category_mode: CategoryMode,
    pub menu_selection: usize,    // 0-2 for difficulty (left column)
    pub variety_selection: usize, // 0-2 for variety (right column)
    pub menu_column: usize,       // 0=left (difficulty), 1=right (variety)
    /// Y position of falling mushroom (0.0 = top, 1.0 = landed)
    pub fall_progress: f64,
    /// Speed: fraction of height per second
    pub fall_speed: f64,
    pub feedback_message: Option<String>,
    next_mushroom_id: usize,
    /// Total baskets collected across levels
    pub total_baskets: usize,
    /// IDs of mushrooms correctly sorted in current level (won't reappear)
    pub sorted_this_level: HashSet<String>,
    /// All mushrooms collected in baskets across the game (for display)
    pub collected_mushrooms: Vec<FallingMushroom>,
    /// Active center animation
    pub animation: Option<CenterAnimation>,
    /// Pending basket fact to show after basket animation completes
    pending_basket_fact: Option<(Vec<FallingMushroom>, String)>,
    active_catalog: Vec<RuntimeCatalogEntry>,
    imported_catalog: Vec<RuntimeCatalogEntry>,
    pub import_summary: Option<ImportSummary>,
    using_imported_catalog: bool,
    pub viewport: Viewport,
    pub dpr: f64,
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
            variety: Variety::Small,
            current_level: 0,
            category_mode: CategoryMode::Color,
            menu_selection: 0,
            variety_selection: 0,
            menu_column: 0,
            fall_progress: 0.0,
            fall_speed: 0.35,
            feedback_message: None,
            next_mushroom_id: 0,
            total_baskets: 0,
            sorted_this_level: HashSet::new(),
            collected_mushrooms: Vec::new(),
            animation: None,
            pending_basket_fact: None,
            active_catalog: built_in_catalog(Variety::Small),
            imported_catalog: Vec::new(),
            import_summary: None,
            using_imported_catalog: false,
            viewport: Viewport::default(),
            dpr: 1.0,
        }
    }

    pub fn feedback_message(&self) -> Option<&str> {
        self.feedback_message.as_deref()
    }

    pub fn active_catalog_len(&self) -> usize {
        self.active_catalog.len()
    }

    pub fn set_viewport(&mut self, viewport: Viewport) {
        self.viewport = viewport;
    }

    pub fn set_dpr(&mut self, dpr: f64) {
        self.dpr = dpr;
    }

    pub fn active_source_label(&self) -> &'static str {
        if self.using_imported_catalog {
            "iNaturalist"
        } else {
            "Curated"
        }
    }

    pub fn active_source_summary(&self) -> Option<String> {
        if self.using_imported_catalog {
            self.import_summary.as_ref().map(|summary| {
                format!(
                    "@{} - {} matched species",
                    summary.user_login, summary.matched_species_count
                )
            })
        } else {
            None
        }
    }

    pub fn menu_import_summary(&self) -> Option<String> {
        self.import_summary.as_ref().map(|summary| {
            format!(
                "Last import: @{} - {} playable species from {} to {}",
                summary.user_login,
                summary.matched_species_count,
                summary.start_date,
                summary.end_date
            )
        })
    }

    pub fn remember_import(
        &mut self,
        imported_catalog: Vec<RuntimeCatalogEntry>,
        summary: ImportSummary,
    ) {
        self.imported_catalog = imported_catalog;
        self.import_summary = Some(summary.clone());
        self.feedback_message = Some(format!(
            "Loaded {} playable iNaturalist species for @{}.",
            summary.matched_species_count, summary.user_login
        ));
        self.phase = GamePhase::Menu;
    }

    pub fn start_imported_game(&mut self) -> Result<(), String> {
        if self.imported_catalog.is_empty() {
            return Err("Import observations first.".to_owned());
        }

        self.difficulty = Difficulty::all()[self.menu_selection];
        self.variety = Variety::all()[self.variety_selection];
        self.using_imported_catalog = true;
        self.current_level = 0;
        self.total_baskets = 0;
        self.collected_mushrooms.clear();
        self.prepare_active_catalog();
        self.start_level();
        Ok(())
    }

    pub fn start_game(&mut self) {
        self.using_imported_catalog = false;
        self.current_level = 0;
        self.total_baskets = 0;
        self.collected_mushrooms.clear();
        self.prepare_active_catalog();
        self.start_level();
    }

    fn prepare_active_catalog(&mut self) {
        self.active_catalog = if self.using_imported_catalog && !self.imported_catalog.is_empty() {
            cap_catalog(&self.imported_catalog, self.variety)
        } else {
            built_in_catalog(self.variety)
        };
    }

    fn start_level(&mut self) {
        self.phase = GamePhase::Playing;
        self.category_mode = CategoryMode::for_level(self.current_level);
        self.sorted_this_level.clear();
        self.game = GameState::new(GameConfig {
            lane_count: self.settings.lane_count,
            points_per_clear: self.settings.points_per_clear,
            points_per_correct: self.settings.points_per_correct,
        })
        .expect("valid config");
        self.next_mushroom_id = 0;
        self.fall_progress = 0.0;
        self.animation = None;
        self.pending_basket_fact = None;
        self.feedback_message = Some(if self.using_imported_catalog {
            format!(
                "Level {} - Sort your iNaturalist fungi by {}!",
                self.current_level + 1,
                self.category_mode.display_name()
            )
        } else {
            format!(
                "Level {} - Sort by {}!",
                self.current_level + 1,
                self.category_mode.display_name()
            )
        });
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

        let mushroom = if let Some(retry) = self.game.pop_retry() {
            retry
        } else {
            match pick_mushroom(
                self.next_mushroom_id,
                self.category_mode,
                &self.active_catalog,
                &self.sorted_this_level,
            ) {
                Some(mushroom) => {
                    self.next_mushroom_id += 1;
                    mushroom
                }
                None => {
                    self.phase = GamePhase::LevelComplete;
                    return;
                }
            }
        };

        self.game
            .spawn(
                mushroom,
                self.settings.spawn_lane.min(self.settings.lane_count - 1),
            )
            .expect("spawn lane stays within configured bounds");
        self.fall_progress = 0.0;
    }

    /// Advance falling animation and center animations. Returns true if mushroom auto-landed.
    pub fn tick(&mut self, dt_seconds: f64) -> bool {
        if let Some(ref mut anim) = self.animation {
            anim.advance(dt_seconds);
            if anim.is_done() {
                let was_basket = matches!(anim, CenterAnimation::BasketCollected { .. });
                self.animation = None;
                if was_basket {
                    if let Some((mushrooms, fact)) = self.pending_basket_fact.take() {
                        self.phase = GamePhase::BasketFact { mushrooms, fact };
                    }
                }
            }
        }

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
                    for mushroom in &feedback.cleared_mushrooms {
                        self.collected_mushrooms.push(mushroom.clone());
                    }
                    let fact = basket_fact(&feedback.cleared_mushrooms);
                    self.animation = Some(CenterAnimation::BasketCollected { progress: 0.0 });
                    self.pending_basket_fact = Some((feedback.cleared_mushrooms, fact));
                    self.feedback_message = Some(format!(
                        "Basket #{} collected! +{} points!",
                        self.total_baskets, feedback.awarded_points
                    ));
                } else if feedback.correct_lane {
                    self.sorted_this_level.insert(feedback.mushroom_id.clone());
                    self.animation = Some(CenterAnimation::Correct { progress: 0.0 });
                    self.feedback_message = Some(format!(
                        "Correct! {} -> basket (+{})",
                        feedback.mushroom_name, feedback.awarded_points
                    ));
                    self.ensure_active_mushroom();
                } else {
                    self.animation = Some(CenterAnimation::Wrong { progress: 0.0 });
                    self.feedback_message =
                        Some(format!("Wrong bucket for {}", feedback.mushroom_name));
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
            GamePhase::LevelComplete => self.handle_level_complete_action(action),
            GamePhase::GameOver => self.handle_gameover_action(action),
        }
    }

    pub fn handle_pointer(&mut self, x: f64, y: f64) {
        match &self.phase {
            GamePhase::Menu => self.handle_menu_pointer(x, y),
            GamePhase::Playing => self.handle_play_pointer(x, y),
            GamePhase::Paused => self.handle_pause_pointer(x, y),
            GamePhase::BasketFact { .. } => self.handle_basket_pointer(x, y),
            GamePhase::LevelComplete => self.handle_level_complete_pointer(x, y),
            GamePhase::GameOver => self.handle_gameover_pointer(x, y),
        }
    }

    fn handle_menu_action(&mut self, action: InputAction) {
        let diff_count = Difficulty::all().len();
        let var_count = Variety::all().len();
        match action {
            InputAction::MoveUp => {
                if self.menu_column == 0 {
                    if self.menu_selection > 0 {
                        self.menu_selection -= 1;
                    }
                } else if self.variety_selection > 0 {
                    self.variety_selection -= 1;
                }
            }
            InputAction::MoveDown => {
                if self.menu_column == 0 {
                    if self.menu_selection + 1 < diff_count {
                        self.menu_selection += 1;
                    }
                } else if self.variety_selection + 1 < var_count {
                    self.variety_selection += 1;
                }
            }
            InputAction::MoveLeft => {
                self.menu_column = 0;
            }
            InputAction::MoveRight => {
                self.menu_column = 1;
            }
            InputAction::HardDrop | InputAction::Confirm => {
                self.difficulty = Difficulty::all()[self.menu_selection];
                self.variety = Variety::all()[self.variety_selection];
                self.start_game();
            }
            _ => {}
        }
    }

    fn handle_menu_pointer(&mut self, x: f64, y: f64) {
        let layout = ui::menu_layout(self.viewport);
        if let Some(index) = layout
            .difficulty_cards
            .iter()
            .position(|card| card.contains(x, y))
        {
            self.menu_column = 0;
            self.menu_selection = index;
            return;
        }

        if let Some(index) = layout
            .variety_cards
            .iter()
            .position(|card| card.contains(x, y))
        {
            self.menu_column = 1;
            self.variety_selection = index;
            return;
        }

        if layout.start_button.contains(x, y) {
            self.difficulty = Difficulty::all()[self.menu_selection];
            self.variety = Variety::all()[self.variety_selection];
            self.start_game();
        }
    }

    fn handle_play_action(&mut self, action: InputAction) {
        match action {
            InputAction::MoveLeft => {
                let _ = self.game.move_left();
            }
            InputAction::MoveRight => {
                let _ = self.game.move_right();
            }
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

    fn handle_play_pointer(&mut self, x: f64, y: f64) {
        let layout = ui::play_layout(self.viewport, self.game.lane_count());
        if layout.pause_button.contains(x, y) {
            self.phase = GamePhase::Paused;
            return;
        }

        if let Some(index) = layout
            .lane_hit_rects
            .iter()
            .position(|rect| rect.contains(x, y))
        {
            self.select_lane_and_drop(index);
        }
    }

    fn handle_pause_action(&mut self, action: InputAction) {
        match action {
            InputAction::Pause | InputAction::Confirm => {
                self.phase = GamePhase::Playing;
            }
            InputAction::HardDrop => {
                self.phase = GamePhase::Menu;
                self.feedback_message = None;
            }
            _ => {}
        }
    }

    fn handle_pause_pointer(&mut self, x: f64, y: f64) {
        let primary = ui::primary_button_rect(self.viewport);
        if primary.contains(x, y) {
            self.phase = GamePhase::Playing;
            return;
        }

        let secondary = ui::secondary_button_rect(self.viewport);
        if secondary.contains(x, y) {
            self.phase = GamePhase::Menu;
            self.feedback_message = None;
        }
    }

    fn handle_basket_action(&mut self, action: InputAction) {
        match action {
            InputAction::HardDrop | InputAction::Confirm => {
                self.phase = GamePhase::Playing;
                self.ensure_active_mushroom();
            }
            _ => {}
        }
    }

    fn handle_basket_pointer(&mut self, x: f64, y: f64) {
        if ui::primary_button_rect(self.viewport).contains(x, y) {
            self.phase = GamePhase::Playing;
            self.ensure_active_mushroom();
        }
    }

    fn handle_level_complete_action(&mut self, action: InputAction) {
        match action {
            InputAction::HardDrop | InputAction::Confirm => {
                self.advance_level();
            }
            _ => {}
        }
    }

    fn handle_level_complete_pointer(&mut self, x: f64, y: f64) {
        if ui::primary_button_rect(self.viewport).contains(x, y) {
            self.advance_level();
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

    fn handle_gameover_pointer(&mut self, x: f64, y: f64) {
        if ui::primary_button_rect(self.viewport).contains(x, y) {
            self.phase = GamePhase::Menu;
            self.feedback_message = None;
        }
    }

    fn select_lane_and_drop(&mut self, lane: usize) {
        if self.game.active_mushroom().is_none() {
            return;
        }

        while self.game.active_lane() < lane {
            let _ = self.game.move_right();
        }
        while self.game.active_lane() > lane {
            let _ = self.game.move_left();
        }

        self.do_drop();
    }
}
