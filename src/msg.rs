// Definicije vseh sporočil
pub enum Msg {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    KeyDown(String),
    KeyUp(String),
    Tick,
}
