use crate::models::gamestate::GameState;
use crate::msg::Msg;
use sauron::Cmd;

pub fn update(game_state: &mut GameState, msg: Msg) -> Cmd<Msg> {
    match msg {
        Msg::MoveLeft => game_state.player.move_left(),
        Msg::MoveRight => game_state.player.move_right(),
        Msg::MoveUp => game_state.player.move_up(),
        Msg::MoveDown => game_state.player.move_down(),
        Msg::KeyDown(key) => {
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
    }
    Cmd::none()
}
