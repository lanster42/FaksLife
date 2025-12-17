use crate::models::gamestate::{GameState, Screen, InteractionState};
use crate::models::player;
use crate::msg::Msg;
use sauron::Cmd;
use web_sys::{window, HtmlAudioElement};
use wasm_bindgen::JsCast;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn update(game_state: &mut GameState, msg: Msg) -> Cmd<Msg> {       //this function will decide how to react to msgs, depending on gamestate (which because of 'mut' we can also modify), and return a command
    match msg {
        Msg::StartPressed => {      //when you click Start, set gamestate screen to StartPressed
            game_state.screen = Screen::StartPressed;
            
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
                    game_state.pressed_keys.insert(key.clone());        //for now we don't really care if we insert it back or not because we're using the KeyDown and KeyUp only for music
                    
                    //handling the Menu:
                    if let InteractionState::MenuOpen { item_index, ref mut selection } = &mut game_state.interaction_state {
                        match key.as_str() {
                            "ArrowUp" | "w" | "W" => {
                                if *selection > 0   {    //this is so we can add more or less than 2 options at every interactive item
                                    *selection -= 1;     //selecting upper option
                                };
                            }
                            "ArrowDown" | "s" | "S" => {
                                let max_index = match item_index {
                                    0 => 1,     //item 0 has 2 options (coffee and tortilla)
                                    1 => 1,     //item 1 has 2 options (smoke and go home)
                                    _ => 0      //all other items for now have only 1 option
                                };

                                if *selection < max_index {
                                    *selection += 1;
                                }
                            }

                            "Enter" => {
                                //applying selection effects (choosing the option)
                                match item_index {
                                    0 => { 
                                        match *selection {      //first object (counter) has 2 options: coffee and tortilla
                                            0 => game_state.buy_coffee(),
                                            1 => game_state.buy_tortilla(),
                                            _ => {},
                                        }
                                     }
                                    1 => { 
                                        match *selection {
                                            0 => game_state.smoke(),
                                            1 => game_state.go_home(),
                                            _ => {},
                                        }
                                    }
                                    _ => {}     //prepared for new interactable objects
                                }
                                //close menu after you choose
                                game_state.interaction_state = InteractionState::None;
                            }
                            "Escape" => {
                                //cancel menu whenever you press escape
                                game_state.interaction_state = InteractionState::None;
                            }
                            _ => {}
                        }
                        return Cmd::none(); // stop movement while menu is open
                    }

                    //open interaction menu on 'f' or 'F':
                    if key == "f" || key == "F" {
                        // Only open menu if not already open
                        if !matches!(game_state.interaction_state, InteractionState::MenuOpen { .. }) {
                            if let Some(item_index) = game_state.player_near_item(40.0) {
                            let item = &game_state.interactive_items[item_index];
                             if item.id == 2 {
                                game_state.interaction_state = InteractionState::Dialogue {
                                    item_index,
                                    node_index: 0,
                                 };
                                 return Cmd::none();
                            }
                            game_state.interaction_state = InteractionState::MenuOpen {
                                 item_index,
                                 selection: 0,
                            };
                        }
                    }
                }
            }

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
                    game_state.nearby_item = game_state.player_near_item(10.0);     //change this threshold if you want it to activate closer/further
                    
/* 
                //this is if we want character animation in the future :)
                    game_state.player.moving = dx != 0.0 || dy != 0.0;
                    if game_state.player.moving {   //if player pressing a movement key:
                        game_state.player.frame = (game_state.player.frame + 1) % 4;    //you cycle through 4 frames (0, 1, 2, 3)
                    } else {    //if the character isn't moving, stop cycling through the frames / stop animating
                        game_state.player.frame = 0;
                    } */
                }
                Msg::SelectDialogueOption(choice_index) => {
                    if let InteractionState::Dialogue { item_index, node_index } =
                        &game_state.interaction_state
                    {
                        let dialogue = GameState::npc_dialogue(
                            game_state.interactive_items[*item_index].id
                        );

                        let node = &dialogue[*node_index];
                        let response = &node.responses[choice_index];

                        match response.next {
                            Some(next_index) => {
                                game_state.interaction_state = InteractionState::Dialogue {
                                    item_index: *item_index,
                                    node_index: next_index,
                                };
                            }
                            None => {
                                game_state.interaction_state = InteractionState::None;
                            }
                        }
                    }
                }

                _ => {}
            }

            Cmd::none()
         },
         Msg::SelectDialogueOption(choice_index) => {
            if let InteractionState::Dialogue { item_index, node_index } =
                &game_state.interaction_state
            {
                let dialogue = GameState::npc_dialogue(
                    game_state.interactive_items[*item_index].id
                );

                let node = &dialogue[*node_index];
                let response = &node.responses[choice_index];

                match response.next {
                    Some(next_index) => {
                        game_state.interaction_state = InteractionState::Dialogue {
                            item_index: *item_index,
                            node_index: next_index,
                        };
                    }
                    None => {
                        game_state.interaction_state = InteractionState::None;
                    }
                }
            }
            return Cmd::none();
        }
 Msg::CloseDialogue => todo!()
    }
}
