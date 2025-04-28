mod models;
mod msg;
mod update;
mod view;

use sauron::prelude::*;
use crate::models::gamestate::GameState;
use crate::msg::Msg;
use crate::update::update;
use crate::view::view;

pub struct Model {
    game_state: GameState,
}

impl Application for Model {
    type MSG = Msg;

    fn init(&mut self) -> Cmd<Self::MSG> {
        Cmd::none()
    }

    fn update(&mut self, msg: Self::MSG) -> Cmd<Self::MSG> {
        update(&mut self.game_state, msg)
    }

    fn view(&self) -> Node<Self::MSG> {
        view(&self.game_state.player)
    }

    fn style(&self) -> Vec<String> {
        vec![]
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    Program::mount_to_body(Model {
        game_state: GameState::new(),  // Tukaj pokličemo konstruktor
    });
}
