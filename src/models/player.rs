#[derive(Debug, Clone, Copy, PartialEq, Eq)]   //this is so we can print, copy, compare values of Smer
pub enum Smer {    //we want to first represent which direction the player is facing/moving in
    Levo,
    Desno,
    Stoji,
}

pub struct Player {    //we need to represent the player's state
    // pub name: String,
    pub x: i32,    //x coordinate on screen, (0,0) is top left
    pub y: i32,
    pub smer: Smer,    //direction of facing
    pub moving: bool,   //is player moving or not
    pub frame: usize,   //picks the current frame
    // pub money: u32,
    // pub attention: u32,
    // pub anxiety: u32,
}

impl Player {       //with this implementation we just want to CREATE a player starting at (x, y) with no movement and frame 0
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            smer: Smer::Stoji,
            moving: false,
            frame: 0,
        }
    }


    //let's move the player by 4 pixels per step in each direction in response to keyboard input
    pub fn move_left(&mut self) {
        self.x -= 4;
    }

    pub fn move_right(&mut self) {
        self.x += 4;
    }

    pub fn move_up(&mut self) {
        self.y -= 4;
    }

    pub fn move_down(&mut self) {
        self.y += 4;
    }


    //for smoother movement let's define move_by
    pub fn move_by(&mut self, dx: f32, dy: f32) {
        self.x = (self.x as f32 + dx).round() as i32;   //first we transform current x position into float (from integer) and then add the new dx (which is a float) and then round the sum to the nearest integer 
        self.y = (self.y as f32 + dy).round() as i32;
    }
}
