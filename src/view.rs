use crate::msg::Msg;
use crate::models::player::{Player, Smer}; // Dodano: Smer
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
            },
        ],
        [
            // Ozadje
            img(
                vec![
                    attr("src", "/static/background/slika.png"),
                    style! {
                        "position" : "absolute",
                        "top": "0px",
                        "left": "0px",
                        "width": "100%",
                        "height": "100%",
                        "z-index": "0",
                        "image-rendering": "pixelated",
                    },
                ],
                vec![],
            ),

            // Igralec (animirana smer levo/desno/stoji)
            {
                let src = match player.smer {
                    Smer::Levo => "/static/characters/lan_levo_0.png",
                    Smer::Desno => "/static/characters/lan_desno_0.png",
                    Smer::Stoji => "/static/characters/gremlin.png",
                };

                img(
                    [
                        attr("src", src),
                        style! {
                            "position": "absolute",
                            "width": "70px",
                            "height": "70px",
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
