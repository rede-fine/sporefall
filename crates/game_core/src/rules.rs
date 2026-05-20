use crate::model::{FallingMushroom, GameConfig, GameError, PlacementFeedback};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    config: GameConfig,
    active_mushroom: Option<FallingMushroom>,
    active_lane: usize,
    /// Each lane holds a stack of settled mushrooms (stacking allowed).
    lanes: Vec<Vec<FallingMushroom>>,
    /// Correctly classified mushrooms collected as rewards.
    basket: Vec<FallingMushroom>,
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
            lanes: vec![Vec::new(); config.lane_count],
            basket: Vec::new(),
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

    /// Returns the stack of mushrooms in each lane.
    pub fn lanes(&self) -> &[Vec<FallingMushroom>] {
        &self.lanes
    }

    /// Returns the collection basket of correctly sorted mushrooms.
    pub fn basket(&self) -> &[FallingMushroom] {
        &self.basket
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

        let correct_lane = mushroom.target_lane == self.active_lane;
        let placed_lane = self.active_lane;
        let mushroom_name = mushroom.display_name.clone();

        // Correct → collect in basket, award points
        let mut awarded_points = if correct_lane {
            self.score += self.config.points_per_correct;
            self.basket.push(mushroom.clone());
            self.config.points_per_correct
        } else {
            0
        };

        // Place in lane regardless
        self.lanes[placed_lane].push(mushroom);

        // Row clears when every lane has at least one entry
        let row_cleared = self.lanes.iter().all(|lane| !lane.is_empty());
        if row_cleared {
            self.score += self.config.points_per_clear;
            awarded_points += self.config.points_per_clear;
            // Remove one mushroom from each lane (bottom of stack)
            for lane in &mut self.lanes {
                lane.remove(0);
            }
        }

        Ok(PlacementFeedback {
            correct_lane,
            row_cleared,
            awarded_points,
            placed_lane,
            mushroom_name,
        })
    }
}
