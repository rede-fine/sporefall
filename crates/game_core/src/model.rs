#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FallingMushroom {
    pub id: String,
    pub display_name: String,
    pub target_lane: usize,
    pub image_key: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameConfig {
    pub lane_count: usize,
    pub points_per_clear: u32,
    pub points_per_correct: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacementFeedback {
    pub correct_lane: bool,
    pub row_cleared: bool,
    pub awarded_points: u32,
    pub placed_lane: usize,
    pub mushroom_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameError {
    InvalidLaneCount,
    InvalidTargetLane,
    ActiveMushroomPresent,
    NoActiveMushroom,
}

/// Represents one category system the player can choose.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BucketSet {
    pub id: String,
    pub name: String,
    pub labels: Vec<String>,
}
