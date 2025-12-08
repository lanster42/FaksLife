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
    //detecting if the screen size has changed:
    pub last_known_width: f64,
    pub last_known_height: f64,

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
    let vw = window.inner_width().unwrap().as_f64().unwrap();
    let vh = window.inner_height().unwrap().as_f64().unwrap();
    (vw, vh)
}

impl GameState {
    pub fn new() -> Self {      //creates a new game state, setting everything to default
        let (vw, vh) = get_screen_size();       //browser screen size in pixels
        let world_width = 1200.0;
        let world_height = 800.0;
        let scale = 1.0;
        let last_known_width = -1.;
        let last_known_height = -1.;

        Self {
            world_width,
            world_height,
            viewport_width: vw,
            viewport_height: vh,
            scale,
            last_known_height,
            last_known_width,
            player: Player::new(100., 100.),
            pressed_keys: HashSet::new(),       //no keys pressed
            music_started: false,       //so the default state is no music
            screen: Screen::Start,
        }
    }
    pub fn update_viewport(&mut self) {
        //avoid recomputation if the dimensions didn't change:
        let (display_width, display_height) = get_screen_size();
        if display_width == self.last_known_width &&
           display_height == self.last_known_height {
            return;
        }
        self.last_known_width = display_width;      //remember current screen as the last known
        self.last_known_height = display_height;

        //game aspect ratio:
        let game_aspect = self.world_width / self.world_height;     //because we have a fixed game, this is always 1200/800

        //we're adding "padding" AKA our container will start 10/2px from the edges of screen so it doesn't clip or scroll:
        let padding = 10.0;
        let container_width = display_width - padding;
        let container_height = display_height - padding;
        let container_aspect = container_width / container_height;      //calculating the new aspect ratio

        //now let's change the display depending on which aspect is bigger (because we don't want to stretch our display AKA change the game_aspect):
        let (scaled_w, scaled_h) = if game_aspect > container_aspect {      //so if container too high
            let w = container_width;        //constrained by width
            let h = w / game_aspect;
            (w, h)
        } else {        //if container too wide
            let h = container_height;       // constrained by height
            let w = h * game_aspect;
            (w, h)
        };
        
        //we can finally adjust the viewpoint (how big the screen displays on the device)
        self.viewport_width = scaled_w;
        self.viewport_height = scaled_h;
        self.scale = scaled_w / self.world_width;       //by remembering how much we scaled the original world_width, we can scale all other objects :)

    }
}
