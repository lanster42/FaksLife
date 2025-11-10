use crate::models::gamestate::{GameState, Screen};
use crate::models::player::Smer;
use crate::msg::Msg;
use sauron::Cmd;
use web_sys::{window, HtmlAudioElement};
use wasm_bindgen::JsCast;

pub fn update(game_state: &mut GameState, msg: Msg) -> Cmd<Msg> {       //this function will decide how to react to msgs, depending on gamestate (which because of 'mut' we can also modify), and return a command
    match msg {
        Msg::StartPressed => {      //when you click Start, set gamestate screen to StartPressed
            game_state.screen = Screen::StartPressed;
            
            //once executed async function:
            Cmd::once(async {       //async is used bc ex. sleep would freeze the entire browser, async pauses the task here, but keeps the app running
                gloo_timers::future::TimeoutFuture::new(300).await;     //how long the StartPressed transition screen stays on
                Msg::StartFinished
            })
        }

        Msg::StartFinished => {
            game_state.screen = Screen::Playing;        //immediately after getting the StartFinished msg, change gamestate.screen to Playing
            Cmd::none()
        }

        Msg::Ignore => Cmd::none(),

        //receiving moving input:
        Msg::MoveLeft
        | Msg::MoveRight
        | Msg::MoveUp
        | Msg::MoveDown
        | Msg::KeyDown(_)
        | Msg::KeyUp(_)
        | Msg::Tick => {
            //if game not in screen Playing, ignore all other events:
            if !matches!(game_state.screen, Screen::Playing) {
                return Cmd::none();
            }

            match msg { //because we want smoother and diagonal movement, ignoring touch screens, this is redundant but we need to remove it from everywhere
                Msg::MoveLeft => game_state.player.move_left(),
                Msg::MoveRight => game_state.player.move_right(),
                Msg::MoveUp => game_state.player.move_up(),
                Msg::MoveDown => game_state.player.move_down(),

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

                    //this part of the code is redundant but it lib has to stop implementing it so we can remove it
                    game_state.pressed_keys.insert(key.clone());    

                    if game_state.pressed_keys.contains("ArrowLeft") || game_state.pressed_keys.contains("a") {
                        game_state.player.move_left();  
                    }
                    if game_state.pressed_keys.contains("ArrowRight") || game_state.pressed_keys.contains("d") {
                        game_state.player.move_right();
                    }
                    if game_state.pressed_keys.contains("ArrowUp") || game_state.pressed_keys.contains("w") {
                        game_state.player.move_up();
                    }
                    if game_state.pressed_keys.contains("ArrowDown") || game_state.pressed_keys.contains("s") {
                        game_state.player.move_down();
                    }
                }

                Msg::KeyUp(key) => {    //we need to remove the key when we stop holding it
                    game_state.pressed_keys.remove(&key);
                }

                Msg::Tick => {      //we add the key to pressed_keys (so we can monitor for more than 1 key pressed at once)
                    let left = game_state.pressed_keys.contains("ArrowLeft") || game_state.pressed_keys.contains("a");
                    let right = game_state.pressed_keys.contains("ArrowRight") || game_state.pressed_keys.contains("d");
                    let up = game_state.pressed_keys.contains("ArrowUp") || game_state.pressed_keys.contains("w");
                    let down = game_state.pressed_keys.contains("ArrowDown") || game_state.pressed_keys.contains("s");

                    let mut dx = 0.0;
                    let mut dy = 0.0;
                    if left {       //calculates the distance (if diagonal we will still need to diagonalize it)
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

                    if dx != 0.0 && dy != 0.0 {     //normalizing the distance
                        let norm = ((dx * dx + dy * dy) as f32).sqrt();
                        dx /= norm;
                        dy /= norm;     //so at this point player moves in all directions at speed 1
                    }

                    let speed = 4.0;        //setting the desired speed, this might be the problematic part why the player moves slower at the start and then speeds up :')
                    dx *= speed;
                    dy *= speed;

                    game_state.player.move_by(dx, dy);      //this should be the correct implementation of the movement :)

                    //changing where player looks depending on movement:
                    if dx < 0.0 {
                        game_state.player.smer = Smer::Levo;
                    } else if dx > 0.0 {
                        game_state.player.smer = Smer::Desno;
                    } /* else if dy > 0.0 {
                        gamestate.player.smer = Smer::Gor;
                    } */else {
                        game_state.player.smer = Smer::Stoji;
                    }

                    //this is where we set character animation :)
                    game_state.player.moving = dx != 0.0 || dy != 0.0;
                    if game_state.player.moving {   //if player pressing a movement key:
                        game_state.player.frame = (game_state.player.frame + 1) % 4;    //you cycle through 4 frames (0, 1, 2, 3)
                    } else {    //if the character isn't moving, stop cycling through the frames / stop animating
                        game_state.player.frame = 0;
                    }
                }

                _ => {}
            }

            Cmd::none()
        }
    }
}
