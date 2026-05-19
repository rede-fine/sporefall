#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FallingMushroom {
    pub id: String,
    pub target_lane: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameConfig {
    pub lane_count: usize,
    pub points_per_clear: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacementFeedback {
    pub correct_lane: bool,
    pub row_cleared: bool,
    pub awarded_points: u32,
    pub placed_lane: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameError {
    InvalidLaneCount,
    InvalidTargetLane,
    ActiveMushroomPresent,
    NoActiveMushroom,
    LaneOccupied,
}

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

        let row_cleared = self.settled_row.iter().all(Option::is_some);
        let awarded_points = if row_cleared {
            self.score += self.config.points_per_clear;
            for slot in &mut self.settled_row {
                *slot = None;
            }
            self.config.points_per_clear
        } else {
            0
        };

        Ok(PlacementFeedback {
            correct_lane,
            row_cleared,
            awarded_points,
            placed_lane,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{FallingMushroom, GameConfig, GameError, GameState};

    fn mushroom(id: &str, target_lane: usize) -> FallingMushroom {
        FallingMushroom {
            id: id.to_owned(),
            target_lane,
        }
    }

    #[test]
    fn rejects_zero_lane_games() {
        let result = GameState::new(GameConfig {
            lane_count: 0,
            points_per_clear: 50,
        });

        assert_eq!(result, Err(GameError::InvalidLaneCount));
    }

    #[test]
    fn movement_stays_inside_lane_bounds() {
        let mut game = GameState::new(GameConfig {
            lane_count: 4,
            points_per_clear: 50,
        })
        .unwrap();

        game.spawn(mushroom("chanterelle", 2), 0).unwrap();
        game.move_left().unwrap();
        assert_eq!(game.active_lane(), 0);

        game.move_right().unwrap();
        game.move_right().unwrap();
        game.move_right().unwrap();
        game.move_right().unwrap();
        assert_eq!(game.active_lane(), 3);
    }

    #[test]
    fn hard_drop_reports_incorrect_lane_without_clearing() {
        let mut game = GameState::new(GameConfig {
            lane_count: 3,
            points_per_clear: 50,
        })
        .unwrap();

        game.spawn(mushroom("fly-agaric", 2), 0).unwrap();
        let feedback = game.hard_drop().unwrap();

        assert!(!feedback.correct_lane);
        assert!(!feedback.row_cleared);
        assert_eq!(feedback.awarded_points, 0);
        assert_eq!(game.score(), 0);
        assert!(game.settled_row()[0].is_some());
    }

    #[test]
    fn filling_the_row_clears_and_scores() {
        let mut game = GameState::new(GameConfig {
            lane_count: 2,
            points_per_clear: 75,
        })
        .unwrap();

        game.spawn(mushroom("boletus", 0), 0).unwrap();
        let first_feedback = game.hard_drop().unwrap();
        assert!(!first_feedback.row_cleared);

        game.spawn(mushroom("morel", 1), 1).unwrap();
        let second_feedback = game.hard_drop().unwrap();

        assert!(second_feedback.correct_lane);
        assert!(second_feedback.row_cleared);
        assert_eq!(second_feedback.awarded_points, 75);
        assert_eq!(game.score(), 75);
        assert!(game.settled_row().iter().all(Option::is_none));
    }
}
