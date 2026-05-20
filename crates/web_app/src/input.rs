#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputAction {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    HardDrop,
    Confirm,
    Pause,
    NextLevel,
}

pub fn map_key_to_action(key: &str) -> Option<InputAction> {
    match key {
        "ArrowLeft" => Some(InputAction::MoveLeft),
        "ArrowRight" => Some(InputAction::MoveRight),
        "ArrowUp" => Some(InputAction::MoveUp),
        "ArrowDown" => Some(InputAction::MoveDown),
        " " | "Space" | "Spacebar" => Some(InputAction::HardDrop),
        "Enter" => Some(InputAction::Confirm),
        "Escape" => Some(InputAction::Pause),
        "n" | "N" => Some(InputAction::NextLevel),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{map_key_to_action, InputAction};

    #[test]
    fn maps_enter_to_confirm() {
        assert_eq!(map_key_to_action("Enter"), Some(InputAction::Confirm));
    }

    #[test]
    fn maps_space_variants_to_hard_drop() {
        assert_eq!(map_key_to_action(" "), Some(InputAction::HardDrop));
        assert_eq!(map_key_to_action("Space"), Some(InputAction::HardDrop));
        assert_eq!(map_key_to_action("Spacebar"), Some(InputAction::HardDrop));
    }
}
