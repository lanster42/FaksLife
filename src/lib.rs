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
    KeyDown(String),
}

impl Application for Model {
    type MSG = Msg;

    fn init(&mut self) -> Cmd<Self::MSG> {
        Cmd::none()
    }
    // kako se spremenijo podatki
    fn update(&mut self, msg: Self::MSG) -> Cmd<Self::MSG> {
        match msg {
            Msg::MoveLeft => self.player_x -= 10,
            Msg::MoveRight => self.player_x += 10,
            Msg::MoveUp => self.player_y -= 10,
            Msg::MoveDown => self.player_y += 10,
            Msg::KeyDown(key) => {
                match key.as_str() {
                    "ArrowLeft" | "a" => self.player_x -= 10,
                    "ArrowRight" | "d" => self.player_x += 10,
                    "ArrowUp" | "w" => self.player_y -= 10,
                    "ArrowDown" | "s" => self.player_y += 10,
                    _ => {}
                }
                
            }
        }
        Cmd::none()
    }
    // kako se narise na zaslon
    fn view(&self) -> Node<Self::MSG> {
        div(
            [
                on_keydown(|event: KeyboardEvent| Msg::KeyDown(event.key())),
                attr("tabindex", "0"),
                style! {
                    "width" : "100vw",
                    "height" : "100vh",
                    "outline" : "none", // da ni cudnga outlinea
                    "overflow" : "hidden", // Naj bi preprečilo premikanje strani
                    "position" : "relative",
                    "background-color" : "green",
                }
            ],
            [
                div(
                    [
                        style! {
                            "position" : "absolute",
                            "width" : "50px",
                            "height" : "50px",
                            "background-color" : "red", //It's christmas
                            "left" : format!("{}px", self.player_x),
                            "top" : format!("{}px", self.player_y),
                        }
                    ],
                    [],
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
        player_x: 100,
        player_y: 100,
    });
}


