use crate::models::gamestate::{GameState, InteractionState, Screen, DialogueNodes, DialogueOutcome, MenuOption};
use crate::models::interactable::{Interactable, Objects, NpcId};
use crate::models::player;
use crate::msg::Msg;
use sauron::Cmd;
use web_sys::{window, HtmlAudioElement};
use wasm_bindgen::JsCast;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn update(game_state: &mut GameState, msg: Msg) -> Cmd<Msg> {       //this function will decide how to react to msgs, depending on gamestate (which because of 'mut' we can also modify), and return a command
    match msg {
        Msg::StartPressed => {      //when you click Start, set gamestate screen to StartPressed
            game_state.screen = Screen::StartPressed;
            game_state.player.money = game_state.player.max_money;      //this resets the player's parameters after game over otherwise the game remembers how much money and anxiety you had at game over 
            game_state.player.anxiety = 0;
            
            //once executed async function:
            Cmd::once(async {       //async is used bc ex. sleep would freeze the entire browser, async pauses the task here, but keeps the app running
                gloo_timers::future::TimeoutFuture::new(300).await;     //how long the StartPressed transition screen stays on
                Msg::StartFinished      //tell the game that the start screen has finished
            })
        }

        Msg::StartFinished => {
            game_state.screen = Screen::Playing;        //immediately after getting the StartFinished msg, change gamestate.screen to Playing
            Cmd::none()
        }

        Msg::Ignore => Cmd::none(),
        
        Msg::Menu => {
            game_state.screen = Screen::Start;        //immediately after getting the StartFinished msg, change gamestate.screen to Playing
            Cmd::none()
        }

        //receiving keyboard input:
        Msg::KeyDown(_)
        | Msg::KeyUp(_)
        | Msg::Tick => {
            //if game not in screen Playing, ignore all other events:
            if !matches!(game_state.screen, Screen::Playing) {
                return Cmd::none();
            }
            match msg {
                Msg::KeyDown(key) => {
                    if !game_state.music_started {  //if music hasn't started yet, it finds the audio element and plays it
                        if let Some(win) = window() {
                            if let Some(doc) = win.document() {
                                if let Some(el) = doc.get_element_by_id("bg-music") {   //audio element
                                    if let Ok(audio) = el.dyn_into::<HtmlAudioElement>() {
                                        let _ = audio.play();
                                    }
                                }
                            }
                        }
                        game_state.music_started = true;
                    }
                    game_state.pressed_keys.insert(key.clone());
                    
            //Handling the Menu:
            if let InteractionState::MenuOpen { interactable, selection } =
                &mut game_state.interaction_state {
                let options = GameState::menu_options_for_item(*interactable);      //get the menu options for this interactable object or NPC
                let current_index = options     //find the index of the current selection in the options
                    .iter()
                    .position(|opt| opt == selection)
                    .unwrap_or(0);

                match key.as_str() {
                    "ArrowUp" | "w" | "W" => {
                        if current_index > 0 {
                            *selection = options[current_index - 1]; //move selection up
                        }
                    }
                    "ArrowDown" | "s" | "S" => {
                        if current_index + 1 < options.len() {
                            *selection = options[current_index + 1]; //move selection down
                        }
                    }
                    "Enter" => {
                        //applying selection effects (choosing the option)
                        match *selection {
                            MenuOption::Coffee => game_state.buy_coffee(),
                            MenuOption::Tortilla => game_state.buy_tortilla(),
                            MenuOption::Smoke => game_state.smoke(),
                            MenuOption::GoHome => game_state.go_home(),
                        }

                        //close menu after selection
                        game_state.interaction_state = InteractionState::None;
                    }
                    "Escape" => {
                        //cancel menu whenever you press escape
                        game_state.interaction_state = InteractionState::None;
                    }
                    _ => {}
                }

                return Cmd::none(); //stop movement while menu is open
            }

            //Open interaction menu on 'f' or 'F':
            if key.eq_ignore_ascii_case("f") {
                //only open menu if not already open
                if !matches!(game_state.interaction_state, InteractionState::MenuOpen { .. }) {
                    //check if the player is near any interactable
                    if let Some(interactable) = game_state.player_near_item(40.0) {
                        match interactable {
                            Interactable::Object(obj) => match obj {
                                Objects::Counter | Objects::Door => {
                                    // Open menu for objects
                                    let options = GameState::menu_options_for_item(interactable);
                                    game_state.interaction_state = InteractionState::MenuOpen {
                                        interactable,
                                        selection: options[0], //start from the first option
                                    };
                                }
                            },
                            Interactable::Npc(npc) => match npc {
                                NpcId::Ema => {
                                    // If we're talking to the only NPC we have for now
                                    game_state.interaction_state = InteractionState::Dialogue {
                                        npc,               //store the NPC
                                        node: DialogueNodes::Živjo, //start dialogue from Živjo
                                    };
                                    return Cmd::none(); //stop movement when dialogue starts
                                }
                            },
                        }
                    }
                }
            }}


                Msg::KeyUp(key) => {    //we need to remove the key when we stop holding it
                    game_state.pressed_keys.remove(&key);
                }

                Msg::Tick => {
                    game_state.update_viewport();       //on every tick we check if the screen size changed

                    //we add the key to pressed_keys (so we can monitor for more than 1 key pressed at once)
                    let left = game_state.pressed_keys.contains("ArrowLeft") || game_state.pressed_keys.contains("a");
                    let right = game_state.pressed_keys.contains("ArrowRight") || game_state.pressed_keys.contains("d");
                    let up = game_state.pressed_keys.contains("ArrowUp") || game_state.pressed_keys.contains("w");
                    let down = game_state.pressed_keys.contains("ArrowDown") || game_state.pressed_keys.contains("s");

                    let mut dx: f64 = 0.0;
                    let mut dy: f64 = 0.0;
                    if left {       //calculates the distance (if diagonal we still need to diagonalize it)
                        dx -= 1.0;
                    }
                    if right {
                        dx += 1.0;
                    }
                    if up {
                        dy -= 1.0;
                    }
                    if down {
                        dy += 1.0;
                    }

                    if dx != 0.0 && dy != 0.0 {     //normalizing the distance if we are moving diagonally
                        let norm = (dx * dx + dy * dy).sqrt();
                        dx /= norm;
                        dy /= norm;     //so at this point player moves in all directions at speed 1
                    }

                    let speed = 5.0;        //setting the desired speed
                    dx *= speed;
                    dy *= speed;

                    let next_x = game_state.player.x + dx;
                    let next_y = game_state.player.y + dy;  //this should be the correct implementation of the movement :)

                    let pw = game_state.player.width;
                    let ph = game_state.player.height;

                    if !game_state.collides_with_wall(next_x, next_y, pw, ph) {
                        game_state.player.move_by(dx, dy); // premakneš se sam če ni stene tm kamor hočeš it
                    }     

                    //changing where player looks depending on movement:
                    if dx < 0.0 {
                        game_state.player.smer = player::Smer::Levo;
                    } else if dx > 0.0 {
                        game_state.player.smer = player::Smer::Desno;
                    } /* else if dy > 0.0 {
                        gamestate.player.smer = player::Smer::Gor;
                    } */else {
                        game_state.player.smer = player::Smer::Stoji;
                    }

                    //setting the screen boundaries AKA preventing player from moving outside of borders
                    let viewport_world_width  = game_state.viewport_width  / game_state.scale;
                    let viewport_world_height = game_state.viewport_height / game_state.scale;

                    game_state.player.x = game_state.player.x.clamp(
                        0.0,
                        viewport_world_width - game_state.player.width,
                    );

                    game_state.player.y = game_state.player.y.clamp(
                        0.0,
                        viewport_world_height - game_state.player.height,
                    );

                    

                    //checking whether we're near enough to an interactive item:
                    game_state.nearby_item = game_state.player_near_item(40.0);     //change this threshold if you want it to activate closer/further
          
                    if game_state.player.anxiety >= game_state.player.max_anxiety {
                        game_state.screen = Screen::GameOver;
                        game_state.interaction_state = InteractionState::None;
                        return Cmd::none(); // konec igre če maxaš out anxiety stat
                    }
/* 
                //this is if we want character animation in the future :)
                    game_state.player.moving = dx != 0.0 || dy != 0.0;
                    if game_state.player.moving {   //if player pressing a movement key:
                        game_state.player.frame = (game_state.player.frame + 1) % 4;    //you cycle through 4 frames (0, 1, 2, 3)
                    } else {    //if the character isn't moving, stop cycling through the frames / stop animating
                        game_state.player.frame = 0;
                    } */
                }


                _ => {}
            }

            Cmd::none()
         },
        
        Msg::SelectDialogueOption(choice_index) => {
            if let InteractionState::Dialogue { npc, node } =
                &game_state.interaction_state
            {
                let dialogue = GameState::npc_dialogue(
                    *npc,
                );

                let current_node = match dialogue.get(node) {
                    Some(n) => n,
                    None => {
                        game_state.interaction_state = InteractionState::None;
                        return Cmd::none();
                    }
                };

                let response = &current_node.responses[choice_index];

                match &response.outcome {
                    DialogueOutcome::Continue(next_node) => {
                        game_state.interaction_state = InteractionState::Dialogue {
                            npc: *npc,
                            node: *next_node,
                        };
                    }

                    DialogueOutcome::EndDialogue => {
                        game_state.interaction_state = InteractionState::None;
                    }

                    DialogueOutcome::EndGame => {
                        game_state.screen = Screen::GameOver;
                        game_state.interaction_state = InteractionState::None;
                    }
                }
            }

            Cmd::none()
        }
    }
}

