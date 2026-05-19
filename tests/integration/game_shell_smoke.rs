#[path = "../../crates/web_app/src/input.rs"]
mod input;

use input::{map_key_to_action, InputAction};

#[test]
fn maps_expected_keyboard_controls() {
    assert_eq!(map_key_to_action("ArrowLeft"), Some(InputAction::MoveLeft));
    assert_eq!(map_key_to_action("ArrowRight"), Some(InputAction::MoveRight));
    assert_eq!(map_key_to_action("Space"), Some(InputAction::HardDrop));
    assert_eq!(map_key_to_action("Enter"), None);
}
