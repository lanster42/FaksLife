//this is where we define the current state of our game and everything that needs to be tracked while game runs

use crate::models::player::Player;
use std::collections::HashSet;  //used to store pressed keys
use web_sys::window;    //so we can get the screen size


pub enum Screen {      //defines which part/screen of your game you're on
    Start,
    StartPressed,   //temporary state after start button is clicked
    Playing,
    //MainMenu,
    //GameOver,
}

//let's define the main struct that basically holds everything about the current game
pub struct GameState {     
    pub world_width: f64,    //the first 4 are for determining the size of screen
    pub world_height: f64,
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub scale: f64,     //so screen size can scale
    pub player: Player,     //everything about the player
    pub pressed_keys: HashSet<String>,      //which keys are pressed
    pub music_started: bool,        //yes/no so it doesn't restart every frame
    pub screen: Screen,     //above enum :)
}


//we want the game to adapt to any window size so we gather the size of the browser window screen
fn get_screen_size() -> (f64, f64) {
    let window = window().unwrap();
    let vw = window.inner_width().unwrap().as_f64().unwrap();
    let vh = window.inner_height().unwrap().as_f64().unwrap();
    (vw, vh)
}

impl GameState {
    pub fn new() -> Self {      //creates a new game state, setting everything to default
        let (vw, vh) = get_screen_size();
        let world_width = 1000.0;
        let world_height = 800.0;
        let scale = (vw / world_width).min(vh / world_height);      //scale so that entire world fits vertically or horizontally

        Self {
            world_width,
            world_height,
            viewport_width: vw,
            viewport_height: vh,
            scale,
            player: Player::new(world_width / 2.0, world_height / 2.0),
            pressed_keys: HashSet::new(),       //no keys pressed
            music_started: false,       //so the default state is no music
            screen: Screen::Start,
        }
    }
}
