//this is where we define the current state of our game and everything that needs to be tracked while game runs

use crate::models::player::Player;
use std::collections::HashSet;  //used to store pressed keys
use web_sys::window;    //so we can get the screen size


pub enum Screen {      //defines which part of your game you're in
    Start,
    StartPressed,   //temporary state after start button is clicked
    Playing,
    //MainMenu,
    //GameOver,
}

//let's define the main struct that basically holds everything about the current game
pub struct GameState {     
    pub player: Player,     //everything about the player
    pub pressed_keys: HashSet<String>,      //which keys are pressed
    pub music_started: bool,        //yes/no so it doesn't restart every frame
    pub screen: Screen,     //above enum :)
}


//we want the game to adapt to any window size so we gather the size of the browser window screen
fn get_screen_size() -> (i32, i32) {
    let window = window().expect("no global `window` exists");
    let width = window.inner_width().unwrap().as_f64().unwrap() as i32;
    let height = window.inner_height().unwrap().as_f64().unwrap() as i32;
    (width, height)     //returns them as integers
}

impl GameState {
    pub fn new() -> Self {      //creates a new game state, setting everything to default
        let (screen_width, screen_height) = get_screen_size();
        let player_width = 64;
        let player_height = 64;

        let x = (screen_width / 2) - (player_width / 2);      //calculating the center of the screen so the player can start there
        let y = (screen_height / 2) - (player_height / 2);

        Self {
            player: Player::new(x, y),
            pressed_keys: HashSet::new(),       //no keys pressed
            music_started: false,       //so the default state is no music
            screen: Screen::Start,
        }
    }
}
