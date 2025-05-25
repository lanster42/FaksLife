use crate::models::player::Player;
use std::collections::HashSet;

pub struct GameState {
    pub player: Player,
    pub pressed_keys: HashSet<String>,
    pub music_started: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            player: Player::new(100, 100),
            pressed_keys: HashSet::new(),
            music_started: false
        }
    }
}
