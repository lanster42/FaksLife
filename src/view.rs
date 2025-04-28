use crate::msg::Msg;
use crate::models::player::Player;
use sauron::prelude::*;

pub fn view(player: &Player) -> Node<Msg> {
    div(
        [
            on_keydown(|event: KeyboardEvent| Msg::KeyDown(event.key())),
            attr("tabindex", "0"),
            style! {
                "width" : "100vw",
                "height" : "100vh",
                "outline" : "none",
                "overflow" : "hidden",
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
                        "background-color" : "red",
                        "left" : format!("{}px", player.x),
                        "top" : format!("{}px", player.y),
                    }
                ],
                [],
            ),
        ],
    )
}
