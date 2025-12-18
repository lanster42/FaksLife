use crate::msg::Msg;
use crate::models::gamestate::{GameState, Screen, InteractionState};
use crate::models::player::Smer;
use sauron::prelude::*;         //sauron library generates the HTML structure from the RUST code :)

fn menu_options_for_item(item_index: usize) -> Vec<&'static str> { // za vsak index interactive objecta ti da opcije
    match item_index {
        0 => vec!["kupi prijetnu kaficu", "kupi tortilijo"], // za pult
        1 => vec!["pojdi na ćik", "pojdi domov"], // za vrata
        2 => vec!["dober dan!!!", "zakaj si tu??"], // za npc 1
        _ => vec!["???"],
    }
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
                        attr("src", "/static/background/start/sunset_start.png"),        //start screen background image
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
                        attr("src", "/static/background/start/start_game.png"),
                        style! {
                            "position": "absolute",
                            "width": format!("{}px", 300. * game_state.scale),      //we're multiplying player's dims and position with scale bc we want to scale it with the window
                            "height": format!("{}px", 180. * game_state.scale),
                            "left": format!("{}px", (game_state.window_width - 300.) * 0.5 * game_state.scale),
                            "top": format!("{}px", (game_state.window_height - 350.) * 0.5 * game_state.scale),    //originalne dimenzije te slike so 75 x 45
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
                        attr("src", "/static/background/start/sunset_start.png"),
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
            let world_left = (game_state.window_width - game_state.viewport_width) / 2.0;       //calculating where the viewport starts so we can spawn the player there and also scale everything correctly
            let world_top  = (game_state.window_height - game_state.viewport_height) / 2.0;

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
                        "background-color" : "black",       //the color of mafija background is "#d5b18c"
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
                                    top: "30px"
                                    left: "30px"
                                    width: format!("{}px", total_width)
                                    height: "20px"
                                    background: "#493508ff"
                                    border: "2px solid black"
                                    z_index: 100
                                    position: "relative"
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
                                ),
                                div(
                [
                    style! {
                        position: "absolute"
                        top: "0"
                        left: "0"
                        width: "100%"
                        height: "100%"
                        display: "flex"
                        align_items: "center"
                        justify_content: "center"
                        font_size: "12px"
                        font_family: "monospace"
                        color: "black"
                        pointer_events: "none"
                    },
                ],
                [text("denar")],
            ),
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
                                    top: "60px"
                                    left: "30px"
                                    width: format!("{}px", total_width)
                                    height: "20px"
                                    background: "#3f275eff"
                                    border: "2px solid black"
                                    z_index: 100
                                    position: "relative"
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
                                ),
                                div(
                                [
                                    style! {
                                        position: "absolute"
                                        top: "0"
                                        left: "0"   
                                        width: "100%"
                                        height: "100%"
                                        display: "flex"
                                        align_items: "center"
                                        justify_content: "center"
                                        font_size: "12px"
                                        font_family: "monospace"
                                        color: "black"
                                        pointer_events: "none"
                                    },
                                ],
                                [text("anksioznost")],
                            ),
            ],
                        )
                    },

                    //Rendering the interactive items:
                    // ================= WORLD CONTAINER =================
                    div(
                        [
                            style! {
                                "position": "absolute",
                                "left": format!("{}px", world_left),
                                "top": format!("{}px", world_top),
                                "width": format!("{}px", game_state.viewport_width),
                                "height": format!("{}px", game_state.viewport_height),
                                "overflow": "hidden",     //we hide overflow so the player can't be rendered outside the world
                            },
                        ],
                        [
                            // Background
                            img(
                                vec![       //vector because the background is already a child and if we want to add ex. table separately, we can do it beside (on top of) the background by defining it as another vector :)
                                    attr("src", "/static/background/mafija_1.png"),
                                    style! {
                                        "position" : "absolute",
                                        "top": "0px",
                                        "left": "0px",
                                        "width": format!("{}px", game_state.viewport_width),
                                        "height": format!("{}px", game_state.viewport_height),
                                        "z-index": "1",
                                        "image-rendering": "pixelated",
                                    },
                                ],
                                vec![],
                            ),
                            // npc
                                 img(
                                    [
                                        attr("src", "/static/characters/ema_naravnost.png",),
                                        style! {
                                            "position": "absolute",
                                            "left": format!("{}px", 500.0 * game_state.scale),      //og dimenzije so 18 x 62
                                            "top": format!("{}px", 500.0 * game_state.scale),   
                                            "width": format!("{}px", 18.0 * game_state.scale),  
                                            "height": format!("{}px", 62.0 * game_state.scale), 
                                            "z-index": "9",                                     
                                            "image-rendering": "pixelated",
                                        },
                                    ],
                                    vec![], 
                                ),

                            // Rendering the interactive items:
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
                                                    //"outline": "1px solid red", //when we're placing the hitbox we want to see its outlined
                                                },
                                            ],
                                            [],
                                            
                                        )
                                    })
                                )
                            },

                            

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
                                            "z-index": "10",
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
                    ),

                    if let InteractionState::Dialogue { item_index, node_index } =
                    &game_state.interaction_state
                {
                    let dialogue = GameState::npc_dialogue(
                        game_state.interactive_items[*item_index].id
                    );
                    let node = &dialogue[*node_index];

                    div(
                        [style! {
                            "position": "absolute",
                            "bottom": "20px",
                            "left": "50%",
                            "transform": "translateX(-50%)",
                            "width": "600px",
                            "background": "#222",
                            "color": "white",
                            "padding": "12px",
                            "z-index": "100",
                        }],
                        [
                            div([], [text(node.text)]),
                            div(
                                [],
                                node.responses.iter().enumerate().map(|(i, r)| {
                                    div(
                                        [
                                            on_click(move |_| Msg::SelectDialogueOption(i)),
                                            style! {
                                                "margin-top": "8px",
                                                "cursor": "pointer",
                                                "background": "#444",
                                                "padding": "6px",
                                            },
                                        ],
                                        [text(r.text)],
                                    )
                                }),
                            ),
                        ],
                    )
                } else {
                    div([], [])
                },

                      // meni za interactive items           
                if let crate::models::gamestate::InteractionState::MenuOpen {
                    item_index,
                    selection,
                } = &game_state.interaction_state
                {
                    let options = menu_options_for_item(*item_index);

                    div(
                        [
                            style! {
                                "position": "absolute",
                                "left": "50%",
                                "top": "50%",
                                "transform": "translate(-50%, -50%)",
                                "background": "#2b1d12",
                                "border": "3px solid black",
                                "padding": "16px",
                                "z-index": "50",
                                "min-width": "220px",
                                "font-family": "monospace",
                                "color": "white",
                            },
                        ],
                        options
                            .iter()
                            .enumerate()
                            .map(|(i, label)| {
                                div(
                                    [
                                        style! {
                                            "padding": "6px 10px",
                                            "margin-bottom": "4px",
                                            "background": if i == *selection {
                                                "#ffdd35"
                                            } else {
                                                "transparent"
                                            },
                                            "color": if i == *selection {
                                                "black"
                                            } else {
                                                "white"
                                            },
                                        },
                                    ],
                                    [text(label)],
                                )
                            }),
                    )
                } else {
                    div([], [])
                }

                ],
            )
        },
        Screen::GameOver =>            
        div(
                [
                    attr("style", "width: 100vw; height: 100vh; overflow: hidden;"),        //prekopiran iz start screena, glej komentarje tam
                ],
                [img(
                    [
                        attr("src", "/static/background/game_over_screen.png"),        
                        style! {
                            "width": "100%",
                            "height": "100%",
                            "object-fit": "cover",      
                            "image-rendering": "pixelated",     
                            "position": "absolute",
                            "top": "0"
                            "left": "0"
                        },
                    ],
                    [],
                ),

                // Press To Start Over image on top of the background
                img(
                [
                        attr("src", "/static/background/start_over_button.png"),
                        style! {
                            "position": "absolute",
                            "width": format!("{}px", 1144. * game_state.scale),      //we're multiplying player's dims and position with scale bc we want to scale it with the window
                            "height": format!("{}px", 80. * game_state.scale),
                            "left": format!("{}px", (game_state.window_width - 1144.) * 0.5 * game_state.scale),
                            "top": format!("{}px", (game_state.window_height - 140.) * 0.5 * game_state.scale),    //originalne dimenzije te slike so 286 x 20
                            "cursor": "pointer",        
                            "image-rendering": "pixelated",
                            "z-index": "10",   
                        },
                    on_click(|_| Msg::Menu),
                ],
                [],
            ),
                ],
        )
        }

    }

