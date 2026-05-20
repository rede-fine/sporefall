use game_core::{FallingMushroom, GameConfig, GameError, GameState};

fn mushroom(id: &str, target_lane: usize) -> FallingMushroom {
    FallingMushroom {
        id: id.to_owned(),
        display_name: id.to_owned(),
        latin_name: format!("{} latinicus", id),
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
    // Wrong placement goes to retry queue, NOT lane
    assert_eq!(game.lanes()[0].len(), 0);
    assert_eq!(game.retry_queue().len(), 1);
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

    // Both correct placements in lane 0
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

    // Place correct in lane 0
    game.spawn(mushroom("boletus", 0), 0).unwrap();
    let first = game.hard_drop().unwrap();
    assert!(!first.row_cleared);
    assert!(first.correct_lane);

    // Place correct in lane 1 — triggers row clear
    game.spawn(mushroom("morel", 1), 1).unwrap();
    let second = game.hard_drop().unwrap();

    assert!(second.correct_lane);
    assert!(second.row_cleared);
    // points_per_correct + points_per_clear = 10 + 75 = 85
    assert_eq!(second.awarded_points, 85);
    // total score: 10 + 10 + 75 = 95
    assert_eq!(game.score(), 95);
    assert!(game.lanes().iter().all(|l| l.is_empty()));
}

#[test]
fn wrong_placement_goes_to_retry_queue_and_can_be_popped() {
    let mut game = GameState::new(GameConfig {
        lane_count: 3,
        points_per_clear: 50,
        points_per_correct: 10,
    })
    .unwrap();

    // Place in wrong lane
    game.spawn(mushroom("fly-agaric", 2), 0).unwrap();
    game.hard_drop().unwrap();

    assert_eq!(game.retry_queue().len(), 1);
    assert_eq!(game.retry_queue()[0].id, "fly-agaric");

    // Pop retry
    let retry = game.pop_retry().unwrap();
    assert_eq!(retry.id, "fly-agaric");
    assert_eq!(retry.target_lane, 2);
    assert!(game.retry_queue().is_empty());
}
