#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputAction {
    MoveLeft,
    MoveRight,
    HardDrop,
}

pub fn map_key_to_action(key: &str) -> Option<InputAction> {
    match key {
        "ArrowLeft" => Some(InputAction::MoveLeft),
        "ArrowRight" => Some(InputAction::MoveRight),
        " " | "Space" | "Spacebar" => Some(InputAction::HardDrop),
        _ => None,
    }
}
