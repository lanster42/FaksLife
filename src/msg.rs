// Definicije vseh sporočil
#[derive(Clone)]
pub enum Msg {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    KeyDown(String),
    KeyUp(String),
    Tick,
    StartPressed,
    StartFinished,
    Ignore,
}
