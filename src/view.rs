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
                        attr("src", "/static/start/start1.png"),        //start screen image
                        style! {
                            "width": "100%",
                            "height": "100%",
                            "object-fit": "cover",      //preserves aspect ratio
                            "image-rendering": "pixelated",     //keeps pixelart sharp (no smoothing)
                            "position": "absolute",
                            "top": "0"
                            "left": "0"
                        },
                        on_click(|mouse_event: MouseEvent| {        //when user clicks on starting screen, check where they clicked and decide whether he 'hit' the start button
                            let x = mouse_event.client_x();     //when user clicks, check the x coord
                            let y = mouse_event.client_y();
                            if x > 100 && x < 300 && y > 100 && y < 200 {       //coordinares of a rectangle of START BUTTON
                                Msg::StartPressed       //if inside rectangle => StartPressed
                            } else {
                                Msg::Ignore     //if missed (outside rectangle) => Ignore :)
                            }
                        }),
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
                    // Ozadje
                    img(
                        vec![
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

                    // Igralec
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
    }
}
