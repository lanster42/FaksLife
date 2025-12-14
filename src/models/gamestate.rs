//this is where we define the current state of our game and everything that needs to be tracked while game runs

use crate::models::player::Player;
use std::collections::HashSet;     //used to store pressed keys
use web_sys::window;    //so we can get the screen size


pub enum Screen {      //defines which part/screen of your game you're on
    Start,
    StartPressed,   //temporary state after start button is clicked
    Playing,
    //MainMenu,
    //GameOver,
}

pub struct Wall {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub struct Item {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    //pub image_path: String,   only used if we have separate pngs for items (not part of the background)
}

pub enum InteractionState {     //enum for interactive items
    None,       //when no interaction is happening
    MenuOpen{
        item_index: usize,      //which object it is
        selection: usize,       //options at object n (AKA 0 = coffee, 1 = tortilla / 0 = smoke, 1 = go home / 0 = study, 1 = go to class)
    }
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
    pub walls: Vec<Wall>, // stene
    pub interactive_items: Vec<Item>,    //vestor of all interactive items
    pub interaction_state: InteractionState,      //when in interaction state
    pub nearby_item: Option<usize>,     //when we detect a nearby item with usize id
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
        let world_width = 1200.0;      //how wide and high the fixed window will be (we're interested in the ratio)
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
            walls: vec![
                Wall { x: 300., y: 200., width: 200., height: 50. },
                Wall { x: 700., y: 400., width: 50., height: 250. },
            ],
            interactive_items: vec![
                Item { id: 0, x: 300., y: 200., width: 200., height: 50. },    //counter
                Item { id: 1, x: 700., y: 400., width: 50., height: 250. },    //bottom door
            ],
            interaction_state: InteractionState::None,
            nearby_item: None
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
        let (scaled_w, scaled_h, new_scale) = if game_aspect > container_aspect {      //so if container too high
            let w = container_width;        //constrained by width
            let h = w / game_aspect;        //we want to always preserve the ratio
            let s = w / self.world_width;
            (w, h, s)
        } else if container_aspect > game_aspect {        //if container too wide
            let h = container_height;       // constrained by height
            let w = h * game_aspect;
            let s = h / self.world_height;
            (w, h, s)
        } else {        //so if the ratios are the same, we can just add the padding
            (container_width, container_height, self.scale)
        };
        
        //we can finally adjust the viewport (how big the screen displays on the device)
        self.viewport_width = scaled_w;
        self.viewport_height = scaled_h;
        self.scale = new_scale;     //scale = new / old;  by remembering how much we scaled the original world_width, we can scale all other objects :)
    }

    pub fn collides_with_wall( // preverja a se hočeš premaknit nekam kjer je stena
        &self,
        next_x: f64,
        next_y: f64,
        pw: f64,
        ph: f64,
    ) -> bool {
        for wall in &self.walls {
            let no_overlap =
                next_x + pw <= wall.x ||           
                next_x >= wall.x + wall.width ||   
                next_y + ph <= wall.y ||          
                next_y >= wall.y + wall.height;

            if !no_overlap {
                return true;    
            }
        }
        false
    }

    pub fn player_near_item(&self, threshold: f64) -> Option<usize> {
        let px_min = self.player.x;
        let px_max = self.player.x + self.player.width;
        let py_min = self.player.y;
        let py_max = self.player.y + self.player.height;

        self.interactive_items
            .iter()
            .filter_map(|item| {
                let ix_min = item.x;
                let ix_max = item.x + item.width;
                let iy_min = item.y;
                let iy_max = item.y + item.height;

                let dx = (ix_min - px_max)
                    .max(px_min - ix_max)
                    .max(0.0);

                let dy = (iy_min - py_max)
                    .max(py_min - iy_max)
                    .max(0.0);

                let dist = (dx * dx + dy * dy).sqrt();

                if dist <= threshold {
                    Some((item.id, dist))
                } else {
                    None
                }
            })
            // choose closest item if multiple are nearby
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(id, _)| id)
    } 


    //INTERACTIVE FUNCTIONS:
    pub fn buy_coffee(&mut self) {
        if self.player.money >= 3 {
            self.player.spend_money(2);
            self.player.get_more_anxious(5);
        }
        else {}
    }

    pub fn buy_tortilla(&mut self) {
        self.player.spend_money(5);
        self.player.get_less_anxious(8);
    }

    pub fn smoke(&mut self) {    //smoking calms you down but maybe there's an increasing chance of having a panic attack (Game Over)
        self.player.get_less_anxious(15);
    }

    pub fn go_home(&mut self) {
        self.screen = Screen::Start; //this should change to /Home in the future when we draw it but now it could be /GameOver
    }
}