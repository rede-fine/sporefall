use crate::model::{FallingMushroom, GameConfig, GameError, PlacementFeedback};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    config: GameConfig,
    active_mushroom: Option<FallingMushroom>,
    active_lane: usize,
    settled_row: Vec<Option<FallingMushroom>>,
    score: u32,
}

impl GameState {
    pub fn new(config: GameConfig) -> Result<Self, GameError> {
        if config.lane_count == 0 {
            return Err(GameError::InvalidLaneCount);
        }

        Ok(Self {
            config,
            active_mushroom: None,
            active_lane: 0,
            settled_row: vec![None; config.lane_count],
            score: 0,
        })
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn lane_count(&self) -> usize {
        self.config.lane_count
    }

    pub fn active_lane(&self) -> usize {
        self.active_lane
    }

    pub fn active_mushroom(&self) -> Option<&FallingMushroom> {
        self.active_mushroom.as_ref()
    }

    pub fn settled_row(&self) -> &[Option<FallingMushroom>] {
        &self.settled_row
    }

    pub fn spawn(&mut self, mushroom: FallingMushroom, starting_lane: usize) -> Result<(), GameError> {
        if self.active_mushroom.is_some() {
            return Err(GameError::ActiveMushroomPresent);
        }
        if mushroom.target_lane >= self.config.lane_count || starting_lane >= self.config.lane_count {
            return Err(GameError::InvalidTargetLane);
        }

        self.active_lane = starting_lane;
        self.active_mushroom = Some(mushroom);
        Ok(())
    }

    pub fn move_left(&mut self) -> Result<(), GameError> {
        if self.active_mushroom.is_none() {
            return Err(GameError::NoActiveMushroom);
        }

        if self.active_lane > 0 {
            self.active_lane -= 1;
        }

        Ok(())
    }

    pub fn move_right(&mut self) -> Result<(), GameError> {
        if self.active_mushroom.is_none() {
            return Err(GameError::NoActiveMushroom);
        }

        if self.active_lane + 1 < self.config.lane_count {
            self.active_lane += 1;
        }

        Ok(())
    }

    pub fn hard_drop(&mut self) -> Result<PlacementFeedback, GameError> {
        let mushroom = self.active_mushroom.take().ok_or(GameError::NoActiveMushroom)?;

        if self.settled_row[self.active_lane].is_some() {
            self.active_mushroom = Some(mushroom);
            return Err(GameError::LaneOccupied);
        }

        let correct_lane = mushroom.target_lane == self.active_lane;
        let placed_lane = self.active_lane;
        self.settled_row[placed_lane] = Some(mushroom);

        let mut awarded_points = if correct_lane {
            self.score += self.config.points_per_correct;
            self.config.points_per_correct
        } else {
            0
        };

        let row_cleared = self.settled_row.iter().all(Option::is_some);
        if row_cleared {
            self.score += self.config.points_per_clear;
            awarded_points += self.config.points_per_clear;
            for slot in &mut self.settled_row {
                *slot = None;
            }
        }

        Ok(PlacementFeedback {
            correct_lane,
            row_cleared,
            awarded_points,
            placed_lane,
        })
    }
}
