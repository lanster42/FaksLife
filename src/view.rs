use crate::msg::Msg;
use crate::models::gamestate::{GameState, Screen};
use crate::models::player::Smer;
use crate::models::player::Player;
use sauron::prelude::*;         //sauron library generates the HTML structure from the RUST code :)

//when we add/draw a player, obstacle or background, you need to multiply the game coordinates by the scale so it scales correctly if the screen dimensions are different:
pub fn render_player(player: &Player, scale: f64) -> Node<Msg> {
    let x = player.x * scale;       //we want the player to scale with the screen
    let y = player.y * scale;
    let w = player.width * scale;
    let h = player.height * scale;

    img(
        vec![
            attr("src", "/static/player/player.png"),
            style! {
                "position": "absolute",
                "left": format!("{}px", x),
                "top": format!("{}px", y),
                "width": format!("{}px", w),
                "height": format!("{}px", h),
                "image-rendering": "pixelated",
                "z-index": "10",
            },
        ],
        vec![],
    )
}



pub fn view(game_state: &GameState) -> Node<Msg> {      //this function will describe what should be shown for the current Gamestate
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
                    attr("id", "game-root"),        //with an id we can refer to it in update
                    style! {        //style of our browser window
                        "position" : "fixed",
                        "top" : "0",
                        "left" : "0",
                        "width" : "100vw",
                        "height" : "100vh",
                        "outline" : "none",
                        "overflow" : "hidden",
                        "background-color" : "pink",
                    },
                ],
                [
                    // Background
                    img(
                        vec![       //vector because the background is already a child and if we want to add ex. table separately, we can do it beside (on top of) the background by defining it as another vector :)
                            attr("src", "/static/background/Kavarna_proba.png"),
                            style! {
                                "position" : "absolute",
                                "top": "50%",
                                "left": "50%",
                                "transform": "translate(-50%, -50%)",
                                "width": format!("{}px", game_state.viewport_width),
                                "height": format!("{}px", game_state.viewport_height),
                               /*  "width": "1200px",       //this was the scale of the picture so I'm keeping it for now so that I can see how I need to change the background picture
                                "border": "3px", */
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
                    render_player(&game_state.player, game_state.scale),
                ],
            )
        }
    }
}
