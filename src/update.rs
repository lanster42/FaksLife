use crate::models::player::Player;
use crate::msg::Msg;
use sauron::Cmd;

pub fn update(player: &mut Player, msg: Msg) -> Cmd<Msg> {
    match msg {
        Msg::MoveLeft => player.move_left(),
        Msg::MoveRight => player.move_right(),
        Msg::MoveUp => player.move_up(),
        Msg::MoveDown => player.move_down(),
        Msg::KeyDown(key) => {
            match key.as_str() {
                "ArrowLeft" | "a" => player.move_left(),
                "ArrowRight" | "d" => player.move_right(),
                "ArrowUp" | "w" => player.move_up(),
                "ArrowDown" | "s" => player.move_down(),
                _ => {}
            }
        }
    }
    Cmd::none()
}
