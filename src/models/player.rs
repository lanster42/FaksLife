#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Smer {
    Levo,
    Desno,
    Stoji,
}

pub struct Player {
    // pub name: String,
    pub x: i32,
    pub y: i32,
    pub smer: Smer,
    pub moving: bool,
    pub frame: usize,
    // pub money: u32,
    // pub attention: u32,
    // pub anxiety: u32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            smer: Smer::Stoji,
            moving: false,
            frame: 0,
        }
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

    pub fn move_by(&mut self, dx: f32, dy: f32) {
        self.x = (self.x as f32 + dx).round() as i32;
        self.y = (self.y as f32 + dy).round() as i32;
    }
}
