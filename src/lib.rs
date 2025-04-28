mod models;
mod msg;
mod update;
mod view;

use crate::models::player::Player;
use crate::msg::Msg;
use sauron::prelude::*;
use crate::update::update;
use crate::view::view;

pub struct Model {
    player: Player,
}

impl Application for Model {
    type MSG = Msg;

    fn init(&mut self) -> Cmd<Self::MSG> {
        Cmd::none()
    }

    fn update(&mut self, msg: Self::MSG) -> Cmd<Self::MSG> {
        update(&mut self.player, msg)
    }

    fn view(&self) -> Node<Self::MSG> {
        view(&self.player)
    }

    fn style(&self) -> Vec<String> {
        vec![]
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    Program::mount_to_body(Model {
        player: Player::new(100, 100),
    });
}
