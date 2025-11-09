use crate::msg::Msg;
use crate::models::gamestate::{GameState, Screen};
use crate::models::player::Smer;
use sauron::prelude::*;         //sauron library generates the HTML structure from the RUST code :)
use web_sys::MouseEvent;

pub fn view(game_state: &GameState) -> Node<Msg> {      //this function will describe what should be shown for the curr. Gamestate
    let player = &game_state.player;

    match game_state.screen {
        Screen::Start => {      //renders when game is at start screen
            div(
                [
                    attr("style", "width: 100vw; height: 100vh; overflow: hidden;"),        //we hide overflow so if picture too big, it will hide the overflown part 
                ],
                [img(
                    [
                        attr("src", "/static/background/start/start1.png"),        //start screen background image
                        style! {
                            "width": "100%",
                            "height": "100%",
                            "object-fit": "cover",      //preserves aspect ratio
                            "image-rendering": "pixelated",     //keeps pixelart sharp (no smoothing)
                            "position": "absolute",
                            "top": "0"
                            "left": "0"
                        },
                    ],
                    [],
                ),

                // Start button image on top of the background
                img(
                [
                        attr("src", "/static/background/start/Start_button_3x_scaled.png"),
                        style! {
                            "position": "absolute",
                            "left": "35%",      //where the start button is located
                            "top": "25%",
                            "width": "384px",   //button size (3x scaled since original image size is 128x32px)
                            "height": "96px",
                            "cursor": "pointer",        //gives you clickable cursor
                            "image-rendering": "pixelated",
                            "z-index": "10",    //makes sure it's on top of the background image
                        },
                    on_click(|_| Msg::StartPressed),
                ],
                [],
            ),
                ],
        )
        }

        //later if we want to add the transition state of the button, we can just add a new msg type and another image :)

        //this is the transition period between start screen and playing screen (this is where the button animation will come)
        Screen::StartPressed => {
            div(
                [
                    attr("style", "width: 100vw; height: 100vh; overflow: hidden;"),
                ],
                [img(
                    [
                        attr("src", "/static/background/start/start1.png"),
                        style! {
                            "width": "100%",
                            "height": "100%",
                            "object-fit": "cover",
                            "image-rendering": "pixelated",
                        },
                    ],
                    [],
                )],
            )
        }

        Screen::Playing => {        //main playing screen where player first spawns (this will maybe be bedroom)
            div(        //everything that has to show together when Playing event
                [
                    on_keydown(|event: KeyboardEvent| Msg::KeyDown(event.key())),   //first screen where you need to listen to keyboard events
                    on_keyup(|event: KeyboardEvent| Msg::KeyUp(event.key())),
                    attr("tabindex", "0"),      //converts value into att so the outer div receives keyboard events
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
                    // Background
                    img(
                        vec![       //vector because the background is already a child and if we want to add ex. table separately, we can do it beside (on top of) the background by defining it as another vector :)
                            attr("src", "/static/background/Kavarna_proba.png"),
                            style! {
                                "position" : "fixed",
                                "top": "50%",
                                "left": "50%",
                                "transform": "translate(-50%, -50%)",
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
                        let src = match player.smer {       //we want to add different images depending on where player is facing
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
    }
}
