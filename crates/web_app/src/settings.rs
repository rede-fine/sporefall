#[derive(Debug, Clone, Copy)]
pub struct PlayerSettings {
    pub lane_count: usize,
    pub points_per_clear: u32,
    pub points_per_correct: u32,
    pub spawn_lane: usize,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            lane_count: 4,
            points_per_clear: 100,
            points_per_correct: 10,
            spawn_lane: 1,
        }
    }
}
