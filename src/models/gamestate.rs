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
    //we'll be using fixed world dimensions:   
    pub world_width: f64,
    pub world_height: f64,

    //device dimensions:
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub scale: f64,     //so screen size can scale
    pub padding: f64,       //adding padding as a pub variable cus we need it for restricting the player

    //later when we have more rooms we can add a spawn point for player depending on room:
    // pub spawn_x: f64,
    // pub spawn_y: f64,

    pub player: Player,     //everything about the player
    pub pressed_keys: HashSet<String>,      //which keys are pressed
    pub music_started: bool,        //yes/no so it doesn't restart every frame
    pub screen: Screen,     //above enum :)
}


//we want the game to adapt to any window size so we gather the size of the browser window screen
fn get_screen_size() -> (f64, f64) {
    let window = window().unwrap();
    let vw = window.inner_width().unwrap().as_f64().unwrap();       //getting size in pixels (floats)
    let vh = window.inner_height().unwrap().as_f64().unwrap();      //btw unwrap returns the value without some. We'll never get None here so it's alright
    (vw, vh)
}

impl GameState {
    pub fn new() -> Self {      //creates a new game state, setting everything to default
        let (vw, vh) = get_screen_size();       //browser screen size in pixels
        let world_width = 1200.0;      //how wide and high the fixed window will be
        let world_height = 600.0;       
        let scale = 1.0;
        let padding = 10.0;

        Self {
            world_width,
            world_height,
            viewport_width: vw,
            viewport_height: vh,
            scale,
            padding,
            player: Player::new(100., 100.),        //where the player spawns, we need to change it so he spawns at the door :)
            pressed_keys: HashSet::new(),       //no keys pressed
            music_started: false,       //so the default state is no music
            screen: Screen::Start,
        }
    }
    pub fn update_viewport(&mut self) {
        //game aspect ratio:
        let game_aspect = self.world_width / self.world_height;     //because we have a fixed game, ratio is always 1200/600
        let (vw, vh) = get_screen_size();

        //we're adding "padding" AKA our container will start 10/2px from the edges of screen so it doesn't clip or scroll:
        let padding = self.padding;
        let container_width = vw - padding;
        let container_height = vh - padding;
        let container_aspect = container_width / container_height;      //calculating the new aspect ratio

        //now let's change the display depending on which aspect is bigger (because we don't want to stretch our display AKA change the game_aspect):
        let (scaled_w, scaled_h) = if game_aspect > container_aspect {      //so if container too high
            let w = container_width;        //constrained by width
            let h = w / game_aspect;        //we want to always preserve the ratio
            (w, h)
        } else if container_aspect > game_aspect {        //if container too wide
            let h = container_height;       // constrained by height
            let w = h * game_aspect;
            (w, h)
        } else {        //so if the ratios are the same, we can just add the padding
            let h = container_height;
            let w = container_width;
            (w, h)
        };
        
        //we can finally adjust the viewpoint (how big the screen displays on the device)
        self.viewport_width = scaled_w;
        self.viewport_height = scaled_h;
        self.scale = scaled_w / self.world_width;       //scale = new / old;  by remembering how much we scaled the original world_width, we can scale all other objects :)
    }
}