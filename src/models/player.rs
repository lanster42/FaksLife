pub struct Player {
    //pub name: String,
    pub x: i32,
    pub y: i32,
    //pub money: u32,
    //pub attention: u32,
    //pub anxiety: u32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player { x, y }
    }

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
}