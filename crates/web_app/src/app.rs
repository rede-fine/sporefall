use game_core::{FallingMushroom, GameConfig, GameState};
use std::collections::HashMap;
use std::collections::HashSet;

use crate::catalog::{
    built_in_catalog, pick_mushroom, CategoryMode, RuntimeCatalogEntry, Variety,
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

/// Per-species performance tracking for the game session.
#[derive(Debug, Clone, Default)]
pub struct SpeciesStats {
    pub correct: u32,
    pub wrong: u32,
}

impl SpeciesStats {
    pub fn accuracy(&self) -> f64 {
        let total = self.correct + self.wrong;
        if total == 0 {
            1.0
        } else {
            self.correct as f64 / total as f64
        }
    }
}

/// Info displayed on a species card overlay.
#[derive(Debug, Clone)]
pub struct SpeciesCard {
    pub id: String,
    pub display_name: String,
    pub latin_name: String,
    pub image_key: String,
    pub ecology: &'static str,
    pub color: &'static str,
    pub season: &'static str,
    pub function: &'static str,
    pub provenance_source: String,
    pub observed_on: Option<String>,
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
    pub using_imported_catalog: bool,
    pub viewport: Viewport,
    pub dpr: f64,
    /// Per-species accuracy tracking for the session
    pub species_stats: HashMap<String, SpeciesStats>,
    /// Rolling accuracy window (last N placements: true=correct, false=wrong)
    accuracy_window: Vec<bool>,
    /// Total correct placements this session
    pub total_correct: u32,
    /// Total wrong placements this session
    pub total_wrong: u32,
    /// Gallery photo index per species (cycles through available photos)
    gallery_index: HashMap<String, usize>,
    /// Species card overlay (shown when user taps a collected mushroom)
    pub species_card: Option<SpeciesCard>,
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
            fall_speed: 0.18,
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
            species_stats: HashMap::new(),
            accuracy_window: Vec::new(),
            total_correct: 0,
            total_wrong: 0,
            gallery_index: HashMap::new(),
            species_card: None,
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
                    "@{} - {} species",
                    summary.user_login, summary.matched_species_count
                )
            })
        } else {
            None
        }
    }

    pub fn has_imported_catalog(&self) -> bool {
        !self.imported_catalog.is_empty()
    }

    pub fn variety_options(&self) -> &'static [Variety] {
        if self.has_imported_catalog() {
            Variety::all_with_inat()
        } else {
            Variety::all()
        }
    }

    pub fn imported_species_count(&self) -> usize {
        self.imported_catalog.len()
    }

    /// Update the imported catalog to only include entries whose IDs are in the set.
    pub fn filter_imported_catalog(&mut self, selected_ids: &HashSet<String>) {
        self.imported_catalog.retain(|e| selected_ids.contains(&e.id));
        if let Some(ref mut summary) = self.import_summary {
            summary.matched_species_count = self.imported_catalog.len();
        }
    }

    pub fn remember_import(
        &mut self,
        imported_catalog: Vec<RuntimeCatalogEntry>,
        summary: ImportSummary,
    ) {
        self.imported_catalog = imported_catalog;
        self.import_summary = Some(summary.clone());
        self.using_imported_catalog = true;
        // Auto-select the iNaturalist variety option (index 3)
        self.variety_selection = 3;
        self.variety = Variety::INaturalist;
        self.feedback_message = Some(format!(
            "Loaded {} iNaturalist species for @{}.",
            summary.matched_species_count, summary.user_login
        ));
        self.phase = GamePhase::Menu;
    }

    pub fn start_game(&mut self) {
        self.using_imported_catalog = self.variety == Variety::INaturalist;
        self.current_level = 0;
        self.total_baskets = 0;
        self.fall_speed = 0.18;
        self.collected_mushrooms.clear();
        self.species_stats.clear();
        self.accuracy_window.clear();
        self.total_correct = 0;
        self.total_wrong = 0;
        self.gallery_index.clear();
        self.prepare_active_catalog();
        self.start_level();
    }

    fn prepare_active_catalog(&mut self) {
        self.active_catalog = if self.using_imported_catalog && !self.imported_catalog.is_empty() {
            // Filter by selected species from the UI checkboxes
            let selected = crate::get_selected_species_ids();
            if selected.is_empty() {
                // If nothing selected (or list not present), use all
                self.imported_catalog.clone()
            } else {
                self.imported_catalog
                    .iter()
                    .filter(|e| selected.contains(&e.id))
                    .cloned()
                    .collect()
            }
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

        let mut mushroom = if let Some(retry) = self.game.pop_retry() {
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

        // Gallery cycling: swap image key to a gallery photo if available
        if self.using_imported_catalog {
            if let Some(catalog_entry) = self.active_catalog.iter().find(|e| e.id == mushroom.id) {
                let gallery = &catalog_entry.provenance.gallery_urls;
                if !gallery.is_empty() {
                    let idx = self.gallery_index.entry(mushroom.id.clone()).or_insert(0);
                    // Cycle: 0 = primary, 1..N = gallery photos
                    *idx += 1;
                    if *idx <= gallery.len() {
                        // Use gallery photo
                        let gallery_key = format!("{}-gallery-{}", catalog_entry.image_key, *idx - 1);
                        mushroom.image_key = gallery_key;
                    } else {
                        // Reset to primary
                        *idx = 0;
                    }
                }
            }
        }

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
                // Track species stats
                let stats = self.species_stats.entry(feedback.mushroom_id.clone()).or_default();

                if feedback.row_cleared {
                    self.total_baskets += 1;
                    self.update_speed_adaptive();
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
                    stats.correct += 1;
                    self.total_correct += 1;
                    self.accuracy_window.push(true);
                    self.update_speed_adaptive();
                    self.sorted_this_level.insert(feedback.mushroom_id.clone());
                    self.animation = Some(CenterAnimation::Correct { progress: 0.0 });
                    self.feedback_message = Some(format!(
                        "Correct! {} -> basket (+{})",
                        feedback.mushroom_name, feedback.awarded_points
                    ));
                    self.ensure_active_mushroom();
                } else {
                    stats.wrong += 1;
                    self.total_wrong += 1;
                    self.accuracy_window.push(false);
                    self.update_speed_adaptive();
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

    /// Adaptive speed: uses rolling accuracy to adjust fall speed.
    /// High accuracy → speed increases faster. Low accuracy → speed decreases.
    fn update_speed_adaptive(&mut self) {
        let window_size = 10;
        // Keep only last N placements
        if self.accuracy_window.len() > window_size {
            let excess = self.accuracy_window.len() - window_size;
            self.accuracy_window.drain(..excess);
        }

        let recent_accuracy = if self.accuracy_window.is_empty() {
            1.0
        } else {
            let correct_count = self.accuracy_window.iter().filter(|&&x| x).count();
            correct_count as f64 / self.accuracy_window.len() as f64
        };

        // Base speed increases with baskets collected
        let base_speed = 0.18 + self.total_baskets as f64 * 0.02;

        // Accuracy multiplier: 0.7 (struggling) to 1.3 (cruising)
        let accuracy_factor = 0.7 + recent_accuracy * 0.6;

        self.fall_speed = (base_speed * accuracy_factor).clamp(0.12, 0.55);
    }

    /// Overall session accuracy (0.0 to 1.0)
    pub fn session_accuracy(&self) -> f64 {
        let total = self.total_correct + self.total_wrong;
        if total == 0 {
            1.0
        } else {
            self.total_correct as f64 / total as f64
        }
    }

    /// Get species sorted by struggle (most wrong first), for the post-game stats.
    pub fn struggled_species(&self) -> Vec<(&str, &SpeciesStats)> {
        let mut species: Vec<(&str, &SpeciesStats)> = self
            .species_stats
            .iter()
            .filter(|(_, stats)| stats.wrong > 0)
            .map(|(id, stats)| (id.as_str(), stats))
            .collect();
        species.sort_by(|a, b| b.1.wrong.cmp(&a.1.wrong).then(a.1.correct.cmp(&b.1.correct)));
        species
    }

    /// Look up display name for a species ID from the active catalog.
    pub fn species_display_name(&self, id: &str) -> String {
        self.active_catalog
            .iter()
            .find(|e| e.id == id)
            .map(|e| e.display_name.clone())
            .unwrap_or_else(|| id.replace('-', " "))
    }

    /// Open a species card overlay for a collected mushroom.
    pub fn open_species_card(&mut self, mushroom_index: usize) {
        let mushroom = match self.collected_mushrooms.get(mushroom_index) {
            Some(m) => m,
            None => return,
        };

        let entry = self.active_catalog.iter().find(|e| e.id == mushroom.id);

        let ecology_labels = CategoryMode::Ecology.labels();
        let color_labels = CategoryMode::Color.labels();
        let season_labels = CategoryMode::Season.labels();
        let function_labels = CategoryMode::Function.labels();

        let (ecology, color, season, function, provenance_source, observed_on) =
            if let Some(entry) = entry {
                (
                    ecology_labels.get(entry.targets[0]).copied().unwrap_or("Unknown"),
                    color_labels.get(entry.targets[1]).copied().unwrap_or("Unknown"),
                    season_labels.get(entry.targets[2]).copied().unwrap_or("Unknown"),
                    function_labels.get(entry.targets[3]).copied().unwrap_or("Unknown"),
                    entry.provenance.source_name.clone(),
                    entry.provenance.observed_on.clone(),
                )
            } else {
                ("Unknown", "Unknown", "Unknown", "Unknown", "Unknown".to_owned(), None)
            };

        self.species_card = Some(SpeciesCard {
            id: mushroom.id.clone(),
            display_name: mushroom.display_name.clone(),
            latin_name: mushroom.latin_name.clone(),
            image_key: mushroom.image_key.clone(),
            ecology,
            color,
            season,
            function,
            provenance_source,
            observed_on,
        });
    }

    /// Dismiss the species card overlay.
    pub fn dismiss_species_card(&mut self) {
        self.species_card = None;
    }

    pub fn handle_action(&mut self, action: InputAction) {
        // Species card overlay intercepts all input (dismiss on any key)
        if self.species_card.is_some() {
            self.dismiss_species_card();
            return;
        }
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
        // Species card overlay: tap anywhere to dismiss
        if self.species_card.is_some() {
            self.dismiss_species_card();
            return;
        }
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
        let var_count = self.variety_options().len();
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
                self.variety = self.variety_options()[self.variety_selection];
                self.start_game();
            }
            _ => {}
        }
    }

    fn handle_menu_pointer(&mut self, x: f64, y: f64) {
        let layout = ui::menu_layout(self.viewport, self.has_imported_catalog());
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
            self.variety = self.variety_options()[self.variety_selection];
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
        if ui::basket_fact_button_rect(self.viewport).contains(x, y) {
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
        // Check collection thumbnails first
        if let Some(idx) = self.collection_thumbnail_at_level_complete(x, y) {
            self.open_species_card(idx);
            return;
        }
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
        // Check collection thumbnails first
        if let Some(idx) = self.collection_thumbnail_at_game_over(x, y) {
            self.open_species_card(idx);
            return;
        }
        if ui::primary_button_rect(self.viewport).contains(x, y) {
            self.phase = GamePhase::Menu;
            self.feedback_message = None;
        }
    }

    /// Hit-test collection grid on level complete screen.
    fn collection_thumbnail_at_level_complete(&self, px: f64, py: f64) -> Option<usize> {
        let v = self.viewport;
        let collection_box = if v.compact {
            ui::Rect { x: 46.0, y: 150.0, width: v.width - 92.0, height: 650.0 }
        } else {
            ui::Rect { x: 58.0, y: 132.0, width: v.width - 116.0, height: 310.0 }
        };
        let thumb_size = if v.compact { 60.0 } else { 46.0 };
        let gap = if v.compact { 18.0 } else { 12.0 };
        let row_height = thumb_size + gap + if v.compact { 16.0 } else { 12.0 };
        let cols = ((collection_box.width - 30.0 + gap) / (thumb_size + gap)).floor().max(1.0) as usize;
        let start_x = collection_box.x + 18.0;
        let start_y = collection_box.y + if v.compact { 50.0 } else { 40.0 };
        self.thumbnail_hit(px, py, start_x, start_y, thumb_size, gap, row_height, cols)
    }

    /// Hit-test collection grid on game over screen.
    fn collection_thumbnail_at_game_over(&self, px: f64, py: f64) -> Option<usize> {
        let v = self.viewport;
        let struggled = self.struggled_species();
        let grid_box = if v.compact {
            ui::Rect { x: 48.0, y: 236.0, width: v.width - 96.0, height: if struggled.is_empty() { 520.0 } else { 320.0 } }
        } else {
            ui::Rect { x: 70.0, y: 240.0, width: v.width - 140.0, height: if struggled.is_empty() { 180.0 } else { 120.0 } }
        };
        let thumb_size = if v.compact { 54.0 } else { 40.0 };
        let gap = if v.compact { 14.0 } else { 10.0 };
        let row_height = thumb_size + gap + if v.compact { 16.0 } else { 12.0 };
        let cols = ((grid_box.width - 24.0 + gap) / (thumb_size + gap)).floor().max(1.0) as usize;
        let start_x = grid_box.x + 12.0;
        let start_y = grid_box.y + 16.0;
        self.thumbnail_hit(px, py, start_x, start_y, thumb_size, gap, row_height, cols)
    }

    fn thumbnail_hit(
        &self, px: f64, py: f64,
        start_x: f64, start_y: f64,
        thumb_size: f64, _gap: f64, row_height: f64, cols: usize,
    ) -> Option<usize> {
        let col = ((px - start_x) / (thumb_size + _gap)).floor() as isize;
        let row = ((py - start_y) / row_height).floor() as isize;
        if col < 0 || row < 0 || col >= cols as isize {
            return None;
        }
        // Check that click is within the thumbnail bounds (not in the gap)
        let thumb_x = start_x + col as f64 * (thumb_size + _gap);
        let thumb_y = start_y + row as f64 * row_height;
        if px < thumb_x || px > thumb_x + thumb_size || py < thumb_y || py > thumb_y + thumb_size {
            return None;
        }
        let index = row as usize * cols + col as usize;
        if index < self.collected_mushrooms.len() {
            Some(index)
        } else {
            None
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
