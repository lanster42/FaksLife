//we will list every kind of event/message the game can receive so when sth happens the right Msg variant is sent to "Update"
#[derive(Clone)]
pub enum Msg {
    KeyDown(String),    //a general event when any key is pressed down
    KeyUp(String),
    Tick,       //represents periodic update ex. one game frame (for animations, movement)
    StartPressed,       //when player clicks Start on start screen
    StartFinished,      //end of start screen to game transition
    Ignore,         //when sth happens you wanna ignore
    Menu,
    SelectDialogueOption(usize),
}
