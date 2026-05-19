use game_core::{FallingMushroom, GameConfig, GameError, GameState};

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
