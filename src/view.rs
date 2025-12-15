use crate::msg::Msg;
use crate::models::gamestate::{GameState, Screen};
use crate::models::player::Smer;
use sauron::prelude::*;         //sauron library generates the HTML structure from the RUST code :)


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

        Screen::Playing => {        //main playing screen where player first spawns (this is for now Mafija)
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
                        "background-color" : "#d5b18c",
                    },
                ],
                [
                    // Money bar
                    {
                        let ratio = player.money as f64 / player.max_money as f64; // kokšen del money bara je pobarvan 
                        let total_width = 200.0;
                        let filled_width = total_width * ratio;

                        div(
                            [
                                style! {
                                    position: "absolute"
                                    top: "30px"
                                    left: "30px"
                                    width: format!("{}px", total_width)
                                    height: "20px"
                                    background: "#493508ff"
                                    border: "2px solid black"
                                    z_index: 100
                                },
                            ],
                            [
                                div(
                                    [
                                        style! {
                                            width: format!("{}px", filled_width)
                                            height: "100%"
                                            background: "#ffdd35ff"
                                        },
                                    ],
                                    [],
                                )
                            ],
                        )
                    },

                    //Anxiety bar
                    {
                        let ratio2 = player.anxiety as f64 / player.max_anxiety as f64; // kokšen del anxiety bara je pobarvan 
                        let total_width = 200.0;
                        let filled_width = total_width * ratio2;

                        div(
                            [
                                style! {
                                    position: "absolute"
                                    top: "60px"
                                    left: "30px"
                                    width: format!("{}px", total_width)
                                    height: "20px"
                                    background: "#3f275eff"
                                    border: "2px solid black"
                                    z_index: 100
                                },
                            ],
                            [
                                div(
                                    [
                                        style! {
                                            width: format!("{}px", filled_width)
                                            height: "100%"
                                            background: "#9335ffff"
                                        },
                                    ],
                                    [],
                                )
                            ],
                        )
                    },

                    //Rendering the interactive items:
                    {// Interactive item hitboxes (invisible)
                        div(
                            [],
                            game_state.interactive_items.iter().map(|item| {
                                div(
                                    [
                                        style! {
                                            "position": "absolute",
                                            "left": format!("{}px", item.x * game_state.scale),
                                            "top": format!("{}px", item.y * game_state.scale),
                                            "width": format!("{}px", item.width * game_state.scale),
                                            "height": format!("{}px", item.height * game_state.scale),
                                            "z-index": "5",
                                            "background": "rgba(0,0,0,0)",
                                            "outline": "1px solid red", //when we're placing the hitbox we want to see its outlined
                                        },
                                    ],
                                    [],
                                )
                            })
                        )
                    },
                    
                    // Background
                    img(
                        vec![       //vector because the background is already a child and if we want to add ex. table separately, we can do it beside (on top of) the background by defining it as another vector :)
                            attr("src", "/static/background/mafija_1.png"),
                            style! {
                                "position" : "absolute",
                                "top": "50%",
                                "left": "50%",
                                "transform": "translate(-50%, -50%)",
                                "width": format!("{}px", game_state.viewport_width),
                                "height": format!("{}px", game_state.viewport_height),
                                "z-index": "1",
                                "image-rendering": "pixelated",
                            },
                        ],
                        vec![],
                    ),

                    // Player
                    {
                        let src = match player.smer {       //we want to add different images depending on where player is facing
                            Smer::Levo => "/static/characters/lan_levo_4x.png",
                            Smer::Desno => "/static/characters/lan_desno_4x.png",
                            Smer::Stoji => "/static/characters/lan_naravnost_4x.png",
                        };

                        img(
                            [
                                attr("src", src),
                                style! {
                                    "position": "absolute",
                                    "width": format!("{}px", player.width * game_state.scale),      //we're multiplying player's dims and position with scale bc we want to scale it with the window
                                    "height": format!("{}px", player.height * game_state.scale),
                                    "left": format!("{}px", player.x * game_state.scale),
                                    "top": format!("{}px", player.y * game_state.scale),
                                    "z-index": "1",
                                    "image-rendering": "pixelated",
                                },
                            ],
                            [],
                        )
                    },

                    // Press F prompt
                    if let Some(item_id) = game_state.nearby_item {
                        let item = &game_state.interactive_items[item_id];

                        img(
                            [
                                attr("src", "/static/background/interactive_objects/F.png"),
                                style! {
                                    "position": "absolute",
                                    "left": format!("{}px", (item.x + item.width / 2.0) * game_state.scale),
                                    "top": format!("{}px", (item.y - 20.0) * game_state.scale),
                                    "width": format!("{}px", 41. * game_state.scale),
                                    "height": format!("{}px", 39. * game_state.scale),
                                    "transform": "translateX(-50%)",
                                    "z-index": "20",
                                    "image-rendering": "pixelated",
                                },
                            ],
                            [],
                        )
                    } else {
                        div([], [])     //empty node
                    },

                ],
            )
        }
    }
}
