use crate::msg::Msg;
use crate::models::player::{Player, Smer};
use sauron::prelude::*;

pub fn view(player: &Player) -> Node<Msg> {
    div(
        [
            on_keydown(|event: KeyboardEvent| Msg::KeyDown(event.key())),
            on_keyup(|event: KeyboardEvent| Msg::KeyUp(event.key())),
            attr("tabindex", "0"),
            style! {
                "width" : "100vw",
                "height" : "100vh",
                "outline" : "none",
                "overflow" : "hidden",
                "position" : "relative",
                "background-color" : "black",
            },
        ],
        [
            // Prostor
            img(
                vec![
                    attr("src", "/static/background/Kavarna_proba.png"),
                    style! {
                        "position" : "fixed",
                        "top": "50%",
                        "left": "50%",
                        "transform": "translate(-50%, -50%)", /* levi rob se bo pomaknu na sredino (da se scentrira) in potem se bo s tansform pomaknu nazaj za polovico sobe. Enako velja za zgornji rob. */
                        "width": "1200px",
                        "border": "3px",
                        "z-index": "1",
                        "image-rendering": "pixelated",
                    },
                ],
                vec![],
            ),

            // Player
            {
                let src = match player.smer {
                    Smer::Levo => "/static/characters/lan_levo.png",
                    Smer::Desno => "/static/characters/lan_desno.png",
                    Smer::Stoji => "/static/characters/lan_naravnost.png",
                };

                img(
                    [
                        attr("src", src),
                        style! {
                            "position": "absolute",
                            "width": "150px",
                            "height": "150px",
                            "left": format!("{}px", player.x),
                            "top": format!("{}px", player.y),
                            "z-index": "1",
                            "image-rendering": "pixelated",
                        },
                    ],
                    [],
                )
            },
        ],
    )
}
