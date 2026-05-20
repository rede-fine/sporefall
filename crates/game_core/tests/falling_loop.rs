use game_core::{FallingMushroom, GameConfig, GameError, GameState};

fn mushroom(id: &str, target_lane: usize) -> FallingMushroom {
    FallingMushroom {
        id: id.to_owned(),
        display_name: id.to_owned(),
        target_lane,
        image_key: id.to_owned(),
    }
}

#[test]
fn rejects_zero_lane_games() {
    let result = GameState::new(GameConfig {
        lane_count: 0,
        points_per_clear: 50,
        points_per_correct: 10,
    });

    assert_eq!(result, Err(GameError::InvalidLaneCount));
}

#[test]
fn movement_stays_inside_lane_bounds() {
    let mut game = GameState::new(GameConfig {
        lane_count: 4,
        points_per_clear: 50,
        points_per_correct: 10,
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
fn correct_placement_awards_points_and_fills_basket() {
    let mut game = GameState::new(GameConfig {
        lane_count: 3,
        points_per_clear: 50,
        points_per_correct: 10,
    })
    .unwrap();

    game.spawn(mushroom("morel", 1), 1).unwrap();
    let feedback = game.hard_drop().unwrap();

    assert!(feedback.correct_lane);
    assert_eq!(feedback.awarded_points, 10);
    assert_eq!(game.basket().len(), 1);
    assert_eq!(game.basket()[0].id, "morel");
}

#[test]
fn incorrect_placement_still_lands_in_lane() {
    let mut game = GameState::new(GameConfig {
        lane_count: 3,
        points_per_clear: 50,
        points_per_correct: 10,
    })
    .unwrap();

    game.spawn(mushroom("fly-agaric", 2), 0).unwrap();
    let feedback = game.hard_drop().unwrap();

    assert!(!feedback.correct_lane);
    assert_eq!(feedback.awarded_points, 0);
    assert_eq!(game.lanes()[0].len(), 1);
    assert!(game.basket().is_empty());
}

#[test]
fn stacking_allowed_in_same_lane() {
    let mut game = GameState::new(GameConfig {
        lane_count: 2,
        points_per_clear: 50,
        points_per_correct: 10,
    })
    .unwrap();

    game.spawn(mushroom("a", 0), 0).unwrap();
    game.hard_drop().unwrap();
    game.spawn(mushroom("b", 0), 0).unwrap();
    game.hard_drop().unwrap();

    assert_eq!(game.lanes()[0].len(), 2);
    assert!(game.lanes()[1].is_empty());
}

#[test]
fn filling_all_lanes_clears_row_and_scores() {
    let mut game = GameState::new(GameConfig {
        lane_count: 2,
        points_per_clear: 75,
        points_per_correct: 10,
    })
    .unwrap();

    game.spawn(mushroom("boletus", 0), 0).unwrap();
    let first = game.hard_drop().unwrap();
    assert!(!first.row_cleared);

    game.spawn(mushroom("morel", 1), 1).unwrap();
    let second = game.hard_drop().unwrap();

    assert!(second.correct_lane);
    assert!(second.row_cleared);
    assert_eq!(second.awarded_points, 85);
    assert_eq!(game.score(), 95);
    assert!(game.lanes().iter().all(|l| l.is_empty()));
}
