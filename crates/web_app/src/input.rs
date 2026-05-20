#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputAction {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    HardDrop,
    Confirm,
}

pub fn map_key_to_action(key: &str) -> Option<InputAction> {
    match key {
        "ArrowLeft" => Some(InputAction::MoveLeft),
        "ArrowRight" => Some(InputAction::MoveRight),
        "ArrowUp" => Some(InputAction::MoveUp),
        "ArrowDown" => Some(InputAction::MoveDown),
        " " | "Space" | "Spacebar" => Some(InputAction::HardDrop),
        "Enter" => Some(InputAction::Confirm),
        _ => None,
    }
}
