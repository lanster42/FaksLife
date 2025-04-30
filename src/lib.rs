mod models;
mod msg;
mod update;
mod view;

use sauron::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;
use std::rc::Rc;
use std::cell::RefCell;
use crate::models::gamestate::GameState;
use crate::msg::Msg;
use crate::update::update;
use crate::view::view;
use wasm_bindgen::closure::Closure;

pub struct Model {
    game_state: GameState,
}

impl Application for Model {
    type MSG = Msg;

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
    let program = Program::mount_to_body(Model {
        game_state: GameState::new(),
    });

    let program = Rc::new(RefCell::new(program));

    let cloned = Rc::clone(&program);
    let closure = Closure::wrap(Box::new(move || {
        cloned.borrow_mut().dispatch(Msg::Tick);
    }) as Box<dyn Fn()>);

    window()
        .unwrap()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            16,
        )
        .unwrap();

    closure.forget();
}
