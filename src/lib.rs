use sauron::prelude::*;

// trenutna pozicija igralca
pub struct Model {
    player_x: i32,
    player_y: i32,
}

// kliki na misko
pub enum Msg {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
}

impl Application for Model {
    type MSG = Msg;

    fn init(&mut self) -> Cmd<Self::MSG> {
        Cmd::none()
    }
    // kako se spremenijo podatki
    fn update(&mut self, msg: Self::MSG) -> Cmd<Self::MSG> {
        match msg {
            Msg::MoveLeft => self.player_x -= 1,
            Msg::MoveRight => self.player_x += 1,
            Msg::MoveUp => self.player_y -= 1,
            Msg::MoveDown => self.player_y += 1,
        }
        Cmd::none()
    }
    // kako se narise na zaslon
    fn view(&self) -> Node<Self::MSG> {
        div(
            [],
            [
                div(
                    [],
                    [text(format!("Igralec: ({}, {})", self.player_x, self.player_y))],
                ),
                div(
                    [],
                    [
                        button([on_click(|_| Msg::MoveLeft)], [text("Levo")]),
                        button([on_click(|_| Msg::MoveRight)], [text("Desno")]),
                        button([on_click(|_| Msg::MoveUp)], [text("Gor")]),
                        button([on_click(|_| Msg::MoveDown)], [text("Dol")]),
                    ],
                ),
            ],
        )
    }

    fn style(&self) -> Vec<String> {
        vec![]
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    Program::mount_to_body(Model {
        player_x: 4,
        player_y: 3,
    });
}
