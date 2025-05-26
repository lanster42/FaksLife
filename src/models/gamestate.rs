use crate::models::player::Player;
use std::collections::HashSet;
use web_sys::window;

pub struct GameState {
    pub player: Player,
    pub pressed_keys: HashSet<String>,
    pub music_started: bool,
}

fn get_screen_size() -> (i32, i32) {
    let window = window().expect("no global `window` exists");
    let width = window.inner_width().unwrap().as_f64().unwrap() as i32;
    let height = window.inner_height().unwrap().as_f64().unwrap() as i32;
    (width, height)
}

impl GameState {
    pub fn new() -> Self {
        let (screen_width, screen_height) = get_screen_size();
        let player_width = 64;
        let player_height = 64;

        let x = (screen_width / 2) - (player_width / 2);
        let y = (screen_height / 2) - (player_height / 2);

        Self {
            player: Player::new(x, y),
            pressed_keys: HashSet::new(),
            music_started: false
        }
    }
}
