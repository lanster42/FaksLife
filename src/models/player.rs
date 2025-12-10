
#[derive(Debug, Clone, Copy, PartialEq, Eq)]   //this is so we can print, copy, compare values of Smer
pub enum Smer {    //we want to first represent which direction the player is facing/moving in
    Levo,
    Desno,
    Stoji,
}

pub struct Player {    //we need to represent the player's state
    // pub name: String,
    pub x: f64,    //x coordinate on screen, (0,0) is top left
    pub y: f64,
    pub smer: Smer,    //direction of facing
    pub width: f64,
    pub height: f64,
    //pub frame: usize,   //picks the current frame
    pub money: i32,
    pub max_money: i32,
    // pub attention: u32,
    pub anxiety: i32,
    pub max_anxiety: i32,
}

impl Player {       //with this implementation we just want to CREATE a player starting at (x, y) with no movement and frame 0
    pub fn new(x: f64, y: f64) -> Self {
        let image_scale = 2.9;
        let width = 20.0 * image_scale;       //these are the dimensions of the image multiplied by a random scale (currently 2.9 bc it matches the background player lol) so if you want to change how big he is just scale it differently
        let height = 62.0 * image_scale;
        Player {
            x,
            y,
            smer: Smer::Stoji,
            width,
            height,
            //frame: 0,   //when we add different frames for the character movement :)
            money: 100,
            max_money: 100,
            anxiety: 0,
            max_anxiety: 100,
        }
    }

    //for smoother movement let's define move_by
    pub fn move_by(&mut self, dx: f64, dy: f64) {
        self.x = self.x + dx;   //moving the player in x direction
        self.y = self.y + dy;
    }

    pub fn spend_money(&mut self, amount: i32) {
    self.money = (self.money - amount).max(0); // ne da se zaslužit keša idk
}

    pub fn get_less_anxious(&mut self, amount: i32) {
    self.anxiety = (self.anxiety - amount).max(0);
    }

    pub fn get_more_anxious(&mut self, amount: i32) {
    self.anxiety = (self.anxiety + amount).min(self.max_anxiety);
}

}
