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
    //pub moving: bool,   //is player moving or not
    //pub frame: usize,   //picks the current frame
    // pub money: u32,
    // pub attention: u32,
    // pub anxiety: u32,
}

impl Player {       //with this implementation we just want to CREATE a player starting at (x, y) with no movement and frame 0
    pub fn new(x: f64, y: f64) -> Self {
        Player {
            x,
            y,
            smer: Smer::Stoji,
            //moving: false,
            //frame: 0,   //when we add different frames for the character movement :)
        }
    }

    //for smoother movement let's define move_by
    pub fn move_by(&mut self, dx: f64, dy: f64) {
        self.x = self.x + dx;   //first we transform current x position into float (from integer) and then add the new dx (which is a float) and then round the sum to the nearest integer 
        self.y = self.y + dy;
    }
}
