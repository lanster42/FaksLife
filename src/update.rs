use crate::models::gamestate::GameState;
use crate::msg::Msg;
use crate::models::player::Smer;
use sauron::Cmd;
use web_sys::window;
use web_sys::HtmlAudioElement;
use wasm_bindgen::JsCast;

pub fn update(game_state: &mut GameState, msg: Msg) -> Cmd<Msg> {
    match msg {
        Msg::MoveLeft => game_state.player.move_left(),
        Msg::MoveRight => game_state.player.move_right(),
        Msg::MoveUp => game_state.player.move_up(),
        Msg::MoveDown => game_state.player.move_down(),
        Msg::KeyDown(key) => {
            if !game_state.music_started {
                if let Some(win) = window() {
                    if let Some(doc) = win.document() {
                        if let Some(el) = doc.get_element_by_id("bg-music") {
                            if let Ok(audio) = el.dyn_into::<HtmlAudioElement>() {
                                let _ = audio.play(); 
                            }
                        }
                    }
                }
            }
                game_state.music_started = true;
            game_state.pressed_keys.insert(key.clone());

            if game_state.pressed_keys.contains("ArrowLeft") || game_state.pressed_keys.contains("a") {
                game_state.player.move_left();
            }
            if game_state.pressed_keys.contains("ArrowRight") || game_state.pressed_keys.contains("d") {
                game_state.player.move_right();
            }
            if game_state.pressed_keys.contains("ArrowUp") || game_state.pressed_keys.contains("w") {
                game_state.player.move_up();
            }
            if game_state.pressed_keys.contains("ArrowDown") || game_state.pressed_keys.contains("s") {
                game_state.player.move_down();
            }
        }
        Msg::KeyUp(key) => {
            game_state.pressed_keys.remove(&key);
        }
        Msg::Tick => {
            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    if let Some(el) = document.get_element_by_id("bg-music") {
                        if let Ok(audio) = el.dyn_into::<HtmlAudioElement>() {
                            let _ = audio.play();
                        }
                    }
                }
            }
            let left = game_state.pressed_keys.contains("ArrowLeft") || game_state.pressed_keys.contains("a");
            let right = game_state.pressed_keys.contains("ArrowRight") || game_state.pressed_keys.contains("d");
            let up = game_state.pressed_keys.contains("ArrowUp") || game_state.pressed_keys.contains("w");
            let down = game_state.pressed_keys.contains("ArrowDown") || game_state.pressed_keys.contains("s");
        
            let mut dx = 0.0;
            let mut dy = 0.0;
            if left {
                dx -= 1.0;
            }
            if right {
                dx += 1.0;
            }
            if up {
                dy -= 1.0;
            }
            if down {
                dy += 1.0;
            }
        
            if dx != 0.0 && dy != 0.0 {
                let norm = ((dx * dx + dy * dy) as f32).sqrt();
                dx /= norm;
                dy /= norm;
            }
        
            let speed = 4.0;
            dx *= speed;
            dy *= speed;
        
            game_state.player.move_by(dx, dy);
      
            if dx < 0.0 {
                game_state.player.smer = Smer::Levo;
            } else if dx > 0.0 {
                game_state.player.smer = Smer::Desno;
            } else {
                game_state.player.smer = Smer::Stoji;
            }
        
            game_state.player.moving = dx != 0.0 || dy != 0.0;
            if game_state.player.moving {
                game_state.player.frame = (game_state.player.frame + 1) % 4;
            } else {
                game_state.player.frame = 0;
            }
        }
    }

    Cmd::none()
}
