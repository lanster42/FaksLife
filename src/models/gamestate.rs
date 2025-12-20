//this is where we define the current state of our game and everything that needs to be tracked while game runs

use crate::models::player::Player;
use crate::models::interactable::{Interactable, NpcId, Objects};
use std::collections::HashSet;     //used to store pressed keys
use std::collections::HashMap;     //used to store dialogue nodes
use web_sys::window;    //so we can get the screen size


pub enum Screen {      //defines which part/screen of your game you're on
    Start,
    StartPressed,   //temporary state after start button is clicked
    Playing,
    //MainMenu,
    GameOver,
}

pub struct Wall {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub struct Item {       //interactive items
    pub kind: Interactable,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    //pub image_path: String,   only used if we have separate pngs for items (not part of the background)
}

pub enum InteractionState {     //enum for interactive items
    None,       //when no interaction is happening
    MenuOpen{
        interactable: Interactable,      //which object it is
        selection: MenuOption,       //options at object n (AKA 0 = coffee, 1 = tortilla / 0 = smoke, 1 = go home / 0 = study, 1 = go to class)
    },
    Dialogue{
        npc: NpcId,
        node: DialogueNodes,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MenuOption {       //all possible menu options we have
    Coffee,
    Tortilla,
    Smoke,
    GoHome,
}

impl MenuOption {
    pub fn label(&self) -> &'static str {    //labels for menu options we want to show on screen
        match self {
            MenuOption::Coffee => "Buy coffee",
            MenuOption::Tortilla => "Buy tortilla",
            MenuOption::Smoke => "Smoke",
            MenuOption::GoHome => "Go home",
        }
    }
}


pub struct DialogueNode {       
    pub text: &'static str,     //what the npc says at current node
    pub responses: Vec<DialogueResponse>,       //which options we have at curr node
}

pub struct DialogueResponse {
    pub text: &'static str,     //what answer we choose (AKA which edge we choose)
    pub outcome: DialogueOutcome,       //what's the outcome after we choose that answer (AKA to which node we move next)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
//enum for dialogue options:
pub enum DialogueNodes {     //start of every npc line
    Živjo,
    AhSajVes,
    Oprosti,
    HvalaLan,
    Ok,
    TiSiTako,
    NeToSe,
    HvalaLanTiSiMoj,
    Ok2,
    UfSeDobro,
    LanASiVRedu,
    LanMarSiIzgubil,
    Poklicala,
    JazSemEma,
    LanResSi,
    EjASiMeVBistvu,
    EjToPaNiRes,
    LepoSlišat,
    KajPočenjaš,
    Ok3,
    AhSiMeŽerestrašil,
    OkSeVidiva,
    OhToRavno,
}

pub enum DialogueOutcome {      //outcomes that can happen after a dialogue option is chosen
    Continue(DialogueNodes),
    EndDialogue,
    EndGame,
}


//let's define the main struct that basically holds everything about the current game
pub struct GameState {
    //we'll be using fixed world dimensions:   
    pub world_width: f64,
    pub world_height: f64,

    //browser window dimensions:
    pub window_width: f64,
    pub window_height: f64,

    //scaled window dimensions:
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub scale: f64,     //so screen size can scale
    pub padding: f64,       //adding padding as a pub variable cus we need it for restricting the player

    //later when we have more rooms we can add a spawn point for player depending on room:
    // pub spawn_x: f64,
    // pub spawn_y: f64,

    pub player: Player,     //everything about the player
    pub pressed_keys: HashSet<String>,      //which keys are pressed
    pub music_started: bool,        //yes/no so it doesn't restart every frame
    pub screen: Screen,     //above enum :)
    pub walls: Vec<Wall>, // stene
    pub interactive_items: Vec<Item>,    //vestor of all interactive items
    pub interaction_state: InteractionState,      //when in interaction state
    pub nearby_item: Option<Interactable>,     //when we detect a nearby item with usize id
}


//we want the game to adapt to any window size so we gather the size of the browser window screen
fn get_screen_size() -> (f64, f64) {
    let window = window().unwrap();
    let vw = window.inner_width().unwrap().as_f64().unwrap();       //getting size in pixels (floats)
    let vh = window.inner_height().unwrap().as_f64().unwrap();      //btw unwrap returns the value without some. We'll never get None here so it's alright
    (vw, vh)
}

impl GameState {
    pub fn new() -> Self {      //creates a new game state, setting everything to default
        let (vw, vh) = get_screen_size();       //browser screen size in pixels
        let world_width = 1200.0;      //how wide and high the fixed window will be (we're interested in the ratio)
        let world_height = 600.0;       
        let scale = 1.0;
        let padding = 10.0;

        Self {
            world_width,
            world_height,
            window_width: vw,
            window_height: vh,
            viewport_width: vw,
            viewport_height: vh,
            scale,
            padding,
            player: Player::new(100., 100.),        //where the player spawns, we need to change it so he spawns at the door :)
            pressed_keys: HashSet::new(),       //no keys pressed
            music_started: false,       //so the default state is no music
            screen: Screen::Start,
            walls: vec![
                Wall { x: 60., y: 0., width: 390., height: 60. }, // pult
                Wall { x: 1003., y: 595., width: 165., height: 12. }, // vrata
                Wall { x: 50., y: 380., width: 80., height: 130. }, // miza spodaj prva
                Wall { x: 260., y: 420., width: 70., height: 100. }, // miza spodaj druga
                Wall { x: 440., y: 415., width: 90., height: 100. }, // miza spodaj tretja
                Wall { x: 700., y: 370., width: 70., height: 200. }, // miza spodaj četrta
                Wall { x: 200., y: 200., width: 100., height: 90. }, // miza zgoraj leva
                Wall { x: 490., y: 190., width: 180., height: 90. }, // miza zgoraj desna
                Wall { x: 530., y: 450., width: 36., height: 124. }, // ema
                Wall { x: 800., y: 0., width: 400., height: 70. }, // pult 2
            ],
            interactive_items: vec![
                Item { kind: Interactable::Object(Objects::Counter), x: 60., y: 10., width: 390., height: 65. },    //counter
                Item { kind: Interactable::Object(Objects::Door), x: 1003., y: 595., width: 165., height: 12. },    //bottom door
                Item { kind: Interactable::Npc(NpcId::Ema), x: 530., y: 450., width: 36., height: 124.}, // Ema as npc has item_id 2
            ],
            interaction_state: InteractionState::None,
            nearby_item: None
        }
    }
    pub fn update_viewport(&mut self) {
        //game aspect ratio:
        let game_aspect = self.world_width / self.world_height;     //because we have a fixed game, ratio is always 1200/600
        let (vw, vh) = get_screen_size();

        //saving the window size so we know what the difference is (AKA where the window starts):
        self.window_width = vw;
        self.window_height = vh;

        //we're adding "padding" AKA our container will start 10/2px from the edges of screen so it doesn't clip or scroll:
        let padding = self.padding;
        let container_width = vw - padding;
        let container_height = vh - padding;
        let container_aspect = container_width / container_height;      //calculating the new aspect ratio

        //now let's change the display depending on which aspect is bigger (because we don't want to stretch our display AKA change the game_aspect):
        let (scaled_w, scaled_h, new_scale) = if game_aspect > container_aspect {      //so if container too high
            let w = container_width;        //constrained by width
            let h = w / game_aspect;        //we want to always preserve the ratio
            let s = w / self.world_width;
            (w, h, s)
        } else if container_aspect > game_aspect {        //if container too wide
            let h = container_height;       // constrained by height
            let w = h * game_aspect;
            let s = h / self.world_height;
            (w, h, s)
        } else {        //so if the ratios are the same, we can just add the padding
            (container_width, container_height, self.scale)
        };
        
        //we can finally adjust the viewport (how big the screen displays on the device)
        self.viewport_width = scaled_w;
        self.viewport_height = scaled_h;
        self.scale = new_scale;     //scale = new / old;  by remembering how much we scaled the original world_width, we can scale all other objects :)
    }

    pub fn collides_with_wall( // preverja a se hočeš premaknit nekam kjer je stena
        &self,
        next_x: f64,
        next_y: f64,
        pw: f64,
        ph: f64,
    ) -> bool {
        for wall in &self.walls {
            let no_overlap =
                next_x + pw <= wall.x ||           
                next_x >= wall.x + wall.width ||   
                next_y + ph <= wall.y ||          
                next_y >= wall.y + wall.height;

            if !no_overlap {
                return true;    
            }
        }
        false
    }

    pub fn player_near_item(&self, threshold: f64) -> Option<Interactable> {
        let px_min = self.player.x;
        let px_max = self.player.x + self.player.width;
        let py_min = self.player.y;
        let py_max = self.player.y + self.player.height;

        self.interactive_items
            .iter()
            .filter_map(|item| {
                let ix_min = item.x;
                let ix_max = item.x + item.width;
                let iy_min = item.y;
                let iy_max = item.y + item.height;

                let dx = (ix_min - px_max)
                    .max(px_min - ix_max)
                    .max(0.0);

                let dy = (iy_min - py_max)
                    .max(py_min - iy_max)
                    .max(0.0);

                let dist = (dx * dx + dy * dy).sqrt();

                if dist <= threshold {
                    Some((item.kind, dist))
                } else {
                    None
                }
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(interactable, _)| interactable)
    }
 


    //INTERACTIVE FUNCTIONS:
    pub fn buy_coffee(&mut self) {
        if self.player.money >= 3 {
            self.player.spend_money(2);
            self.player.get_more_anxious(5);
        }
        else {}
    }

    pub fn buy_tortilla(&mut self) {
        self.player.spend_money(5);
        self.player.get_less_anxious(8);
    }

    pub fn smoke(&mut self) {    //smoking calms you down but maybe there's an increasing chance of having a panic attack (Game Over)
        self.player.get_less_anxious(15);
    }

    pub fn go_home(&mut self) {
        self.screen = Screen::GameOver; //this should change to /Home in the future when we draw it but now it could be /GameOver
    }

    pub fn menu_options_for_item(interactable: Interactable) -> Vec<MenuOption> {    //only handling the menu items here (not dialogue or anything)
        match interactable {
            Interactable::Object(Objects::Counter) => vec![MenuOption::Coffee, MenuOption::Tortilla],        //the null object still remains to be the counter
            Interactable::Object(Objects::Door) => vec![MenuOption::Smoke, MenuOption::GoHome],       //the first interactive object is the door
            _ => vec![],
        }
    }



pub fn npc_dialogue(npc: NpcId) -> HashMap<DialogueNodes, DialogueNode> {       //we instead opt now for a hash map because vectors are usually used when we have something ordered (linearly) which isn't the case here since we have kind of like a directed graph (which isn't ordered)
    let mut map = HashMap::new();   //initiating a new hashmap

    match npc {
        NpcId::Ema => {      //now that we only have one npc this isn't really important but when we add more we can match with other item_ids
            map.insert(
                DialogueNodes::Živjo,   //node label
                DialogueNode {
                    text: "Živjo Lan!!!",   //text under node label
                    responses: vec![
                        DialogueResponse {      //top most response option
                            text: "Živjo Ema!!",    //text it shows for this option
                            outcome: DialogueOutcome::Continue(DialogueNodes::KajPočenjaš),     //what the outcome is if you choose it
                        },
                        DialogueResponse {      //second most top response option
                            text: "Ema! Kaj delaš tukaj?",
                            outcome: DialogueOutcome::Continue(DialogueNodes::AhSajVes),
                        },
                        DialogueResponse {
                            text: "Kdo si ti?",
                            outcome: DialogueOutcome::Continue(DialogueNodes::LanASiVRedu),
                        },
                    ],
                },
            );

            map.insert(
                DialogueNodes::AhSajVes,
                DialogueNode {
                    text: "Ah, saj veš, morala bi delat projektno nalogo za Programiranje 2, ampak raje sedim tu in pijem kavo.",
                    responses: vec![
                        DialogueResponse {
                            text: "J***m ti mater, Ema, zakaj samo jaz delam to projektno??.",
                            outcome: DialogueOutcome::Continue(DialogueNodes::Oprosti),
                        },
                        DialogueResponse {
                            text: "Uživaj, življenje je kratko.",
                            outcome: DialogueOutcome::Continue(DialogueNodes::TiSiTako),
                        },
                    ],
                },
            );

            map.insert(
                DialogueNodes::Oprosti,
                DialogueNode {
                    text: "Oprosti!!! :( Obljubim, da bom jutri zares začela!",
                    responses: vec![
                        DialogueResponse {
                            text: "V redu je, oprostim ti.",
                            outcome: DialogueOutcome::Continue(DialogueNodes::HvalaLan),
                        },
                        DialogueResponse {
                            text: "Ne oprostim ti.",
                            outcome: DialogueOutcome::Continue(DialogueNodes::Ok),
                        },
                    ],
                },
            );

            map.insert(
                DialogueNodes::HvalaLan,
                DialogueNode {
                    text: "Hvala, Lan! Sem vedela, da se lahko zanesem nate. <3",
                    responses: vec![
                        DialogueResponse {
                            text: "Ni za kaj. Zdaj pa pojdi delat!!!",
                            outcome: DialogueOutcome::EndDialogue,
                        },
                    ],
                },
            );

            map.insert(
                DialogueNodes::Ok,
                DialogueNode {
                    text: "Ok :(",
                    responses: vec![
                        DialogueResponse {
                            text: "Saj sem se samo hecal.",
                            outcome: DialogueOutcome::Continue(DialogueNodes::HvalaLan),
                        },
                        DialogueResponse {
                            text: "Zdaj bom šel stran, ker te ne maram.",
                            outcome: DialogueOutcome::EndDialogue,
                        },
                    ],
                },
            );

            map.insert(
                DialogueNodes::TiSiTako,
                DialogueNode {
                    text: "Ti si tako pameten! Kaj slabega pa bi se sploh lahko zgodilo, če odlagam vse svoje delo do zadnjega trenutka?",
                    responses: vec![
                        DialogueResponse {
                            text: "Morda bova zaradi tega dobila slabšo oceno.",
                            outcome: DialogueOutcome::Continue(DialogueNodes::NeToSe),
                        },
                        DialogueResponse {
                            text: "Dobesedno nič.",
                            outcome: DialogueOutcome::Continue(DialogueNodes::HvalaLanTiSiMoj),
                        },
                    ],
                },
            );

            map.insert(
                DialogueNodes::NeToSe,
                DialogueNode {
                    text: "Ne, to se gotovo ne bo zgodilo, saj bom jaz zadnji dan pred rokom napisala tako dober NPC dialog, da bova še vseeno dobila 10.",
                    responses: vec![
                        DialogueResponse {
                            text: "Uau, kako dobra ideja, to bo gotovo delovalo!",
                            outcome: DialogueOutcome::Continue(DialogueNodes::HvalaLanTiSiMoj),
                        },
                        DialogueResponse {
                            text: "Ema, to se ne bo zgodilo.",
                            outcome: DialogueOutcome::Continue(DialogueNodes::Ok2),
                        },
                    ],
                },
            );

            map.insert(
                DialogueNodes::HvalaLanTiSiMoj,
                DialogueNode {
                    text: "Hvala, Lan, ti si moj največji podpornik!",
                    responses: vec![
                        DialogueResponse {
                            text: "Itak, da sem. No, uživaj, moram it.",
                            outcome: DialogueOutcome::EndDialogue,
                        },
                    ],
                },
            );

            map.insert(
                DialogueNodes::Ok2,
                DialogueNode {
                    text: "Ok :(",
                    responses: vec![
                        DialogueResponse {
                            text: "Saj sem se samo hecal.",
                            outcome: DialogueOutcome::Continue(DialogueNodes::HvalaLanTiSiMoj),
                        },
                        DialogueResponse { text: "Zdaj bom šel stran, ker te ne maram.", outcome: DialogueOutcome::EndDialogue},
                    ],
                },
            );

            map.insert(
                DialogueNodes::UfSeDobro,
                DialogueNode {
                    text: "Uf, še dobro. Tole je bilo zdaj malo čudno. A se počutiš v redu?",
                    responses: vec![
                        DialogueResponse { text: "Ja.", outcome: DialogueOutcome::Continue(DialogueNodes::LepoSlišat)},  //tuki bi blo zabavn če se ti pokaže drgačn response če je tvoj anxiety too high
                        DialogueResponse { text: "Ne, zelo se mi vrti, mislim, da bom omedlel.", outcome: DialogueOutcome::Continue(DialogueNodes::Poklicala)},
                    ],
                },
            );

            map.insert(
                DialogueNodes::LanASiVRedu,
                DialogueNode {
                    text: "Lan? A si v redu? Jaz sem Ema, tvoja prijateljica!",
                    responses: vec![
                        DialogueResponse { text: "Ah, seveda, saj res.", outcome: DialogueOutcome::Continue(DialogueNodes::UfSeDobro)}, 
                        DialogueResponse { text: "Kaj? Prvič slišim zate.", outcome: DialogueOutcome::Continue(DialogueNodes::LanMarSiIzgubil)},
                ],
                },
            );

            map.insert(
                DialogueNodes::LanMarSiIzgubil,
                DialogueNode {
                    text: "Lan?? Mar si izgubil spomin? Ali veš, kdo si in kje si?",
                    responses: vec![
                        DialogueResponse { text: "Ja, jaz sem Lan in sem v Mafiji, vsega se spomnem normalno, samo tebe ne. Povej mi več o sebi.", outcome: DialogueOutcome::Continue(DialogueNodes::JazSemEma)},
                        DialogueResponse { text: "V bistvu ne...", outcome: DialogueOutcome::Continue(DialogueNodes::LanResSi)},
                    ],
                },
            );

            map.insert(
                DialogueNodes::Poklicala,
                DialogueNode {
                    text: "Poklicala bom rešilca",
                    responses: vec![
                        DialogueResponse { text: "Ok.", outcome: DialogueOutcome::EndGame}, 
                    ],
                },
            );

            map.insert(
                DialogueNodes::JazSemEma,
                DialogueNode {
                    text: "Jaz sem Ema, spoznala sva se na FMF, kjer sva sošolca že dve leti. Povsod sediva skupaj. Z Borom imamo tekaški klub. Skupaj delava projektno za Programiranje 2...",
                    responses: vec![
                        DialogueResponse { text: "Ah, seveda, saj res.", outcome: DialogueOutcome::Continue(DialogueNodes::UfSeDobro)},  
                        DialogueResponse { text: "To ni mogoče, spomnim se, da sem celo projektno za Programiranje 2 napisal sam.", outcome: DialogueOutcome::Continue(DialogueNodes::EjASiMeVBistvu)}, 
                        DialogueResponse { text: "Kdo je Bor?", outcome: DialogueOutcome::Continue(DialogueNodes::LanResSi)},
                    ],
                },
            );

            map.insert(
                DialogueNodes::LanResSi,
                DialogueNode {
                    text: "Lan!! Res si izgubil spomin!! Poklicala bom rešilca.",
                    responses: vec![
                        DialogueResponse { text: "Mogoče je tako res bolje.", outcome: DialogueOutcome::EndGame},  
                    ],
                },
            );

            map.insert(
                DialogueNodes::EjASiMeVBistvu,
                DialogueNode { // some(15)
                    text: "Ej! A se me v bistvu spomneš, in me samo zafrkavaš, ker se ti zdi, da sem premalo naredila?",
                    responses: vec![
                        DialogueResponse { text: "Ja.", outcome: DialogueOutcome::Continue(DialogueNodes::EjToPaNiRes)},  
                        DialogueResponse { text: "Ne, res ne vem, kdo naj bi ti bila.", outcome: DialogueOutcome::Continue(DialogueNodes::LanResSi)},
                    ],
                },
            );

            map.insert(
                DialogueNodes::EjToPaNiRes,
                DialogueNode { // some(16)
                    text: "Ej!! To pa ni res!! Jaz sem naredila en commit na readme-ju!!",
                    responses: vec![
                        DialogueResponse { text: "To mi nič ne pomeni.", outcome: DialogueOutcome::Continue(DialogueNodes::Oprosti),},  
                        DialogueResponse { text: "Prav imaš. V bistvu si super soprogramerka.", outcome: DialogueOutcome::Continue(DialogueNodes::HvalaLan)},
                    ],
                },
            );

            map.insert(
                DialogueNodes::LepoSlišat,
                DialogueNode { // some(17)
                    text: "Lepo slišat! Kaj pa počenjaš tu?",
                    responses: vec![
                        DialogueResponse { text: "Pijem kavo in hodim okrog.", outcome: DialogueOutcome::Continue(DialogueNodes::OhToRavno)},
                        DialogueResponse { text: "V bistvu sem hotel iti stran od tebe.", outcome: DialogueOutcome::Continue(DialogueNodes::Ok3)},
                    ],
                },
            );

            map.insert(
                DialogueNodes::KajPočenjaš,
                DialogueNode {
                    text: "Kaj počenjaš tu?",
                    responses: vec![
                        DialogueResponse { text: "Pijem kavo in hodim okrog.", outcome: DialogueOutcome::Continue(DialogueNodes::OhToRavno)}, 
                        DialogueResponse { text: "V bistvu sem hotel iti stran od tebe.", outcome: DialogueOutcome::Continue(DialogueNodes::Ok3)},
                    ],
                },
            );

            map.insert(
                DialogueNodes::Ok3,
                DialogueNode {
                    text: "Ok :(((",
                    responses: vec![
                        DialogueResponse { text: "Saj sem se samo hecal.", outcome: DialogueOutcome::Continue(DialogueNodes::AhSiMeŽerestrašil)},
                        DialogueResponse { text: "Zdaj bom šel stran, ker te ne maram.", outcome: DialogueOutcome::EndDialogue},
                    ],
                },
            );

            map.insert(
                DialogueNodes::AhSiMeŽerestrašil,
                DialogueNode {
                    text: "Ah, si me že prestrašil. Boš prisedel?",
                    responses: vec![
                        DialogueResponse { text: "Lahko, samo naj si grem najprej še po eno kavo.", outcome: DialogueOutcome::Continue(DialogueNodes::OkSeVidiva)},  
                        DialogueResponse { text: "V bistvu moram zares nekam iti.", outcome: DialogueOutcome::Continue(DialogueNodes::OhToRavno)}, 
                    ],
                },
            );

            map.insert(
                DialogueNodes::OkSeVidiva,
                DialogueNode {
                    text: "Ok, se vidiva!",
                    responses: vec![
                        DialogueResponse { text: "Ciao.", outcome: DialogueOutcome::EndDialogue},  
                    ],
                },
            );

            map.insert(
                DialogueNodes::OhToRavno,
                DialogueNode {
                    text: "Oh, to ravno počnem tudi jaz, samo da sedim, namesto stojim. Boš prisedel?",
                    responses: vec![
                        DialogueResponse { text: "Lahko, samo naj si grem najprej še po eno kavo.", outcome: DialogueOutcome::Continue(DialogueNodes::OkSeVidiva)},  
                        DialogueResponse { text: "V bistvu moram zares nekam iti.", outcome: DialogueOutcome::Continue(DialogueNodes::OkSeVidiva)}, 
                    ],
                },
            );
        }

        _ => {}
    }

    map
}


}