mod models;
mod msg;
mod update;
mod view;

use sauron::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;
use std::rc::Rc;    //this one and RefCell used for shared mutable state bc Closures cant own Program
use std::cell::RefCell;

use crate::models::gamestate::GameState;    //these are so we don't need to keep typing the whole path
use crate::msg::Msg;
use crate::update::update;
use crate::view::view;
use wasm_bindgen::closure::Closure;

pub struct Model {      //app's main state container
    game_state: GameState,
}

impl Application for Model {
    type MSG = Msg;     //basically telling the app what type of messages it reacts to

    fn update(&mut self, msg: Self::MSG) -> Cmd<Self::MSG> {        //what to do when message happens: you execute the update()
        update(&mut self.game_state, msg)
    }

    fn view(&self) -> Node<Self::MSG> {
        // 🔧 popravljen klic funkcije view
        view(&self.game_state)
    }

    fn style(&self) -> Vec<String> {        //optionally if we want to change the CSS of the generated HTML of the game, we can do it directly :)
        vec![]
    }   //also if we wanted to change the background of the browser or sth we can make a seperate function that matches the styles to the screens for ex. :)
}

#[wasm_bindgen(start)]      //so that the function start() runs immediately when the game generates
pub fn start() {
    let program = Program::mount_to_body(Model {        //creates the app and attaches it to (the body of) HTML
        game_state: GameState::new(),});        //we start with a fresh gamestate. new() is a function (in gamestate mod) that sets everything to default (ex. screen::Start)

    let program = Rc::new(RefCell::new(program));       //we want different pieces of code modify same program

    let cloned = Rc::clone(&program);       //we need to clone it otherwise it won't have it after start() finishes (lifetimes)
    let tick_closure = Closure::wrap(Box::new(move || {      //closure::wrap turns rust code into JS
        cloned.borrow_mut().dispatch(Msg::Tick);        //dispatch sends Tick message each time game updates (which means we can use it for smooth movement)
    }) as Box<dyn Fn()>);

    //we want to run game's loop every 16 ms (AKA 60 FPS) so it sends a tisk 60 times/sec
    window()
        .unwrap()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            tick_closure.as_ref().unchecked_ref(),
            16,
        )
        .unwrap();

    tick_closure.forget();       //so we can keep looping start() until our game runs
}
