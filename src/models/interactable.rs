#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Objects {     //all possible interactable objects we have
    Counter,
    Door,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Interactable {     //differentiating between objects and npcs
    Object(Objects),
    Npc(NpcId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NpcId {    //a least for now we'll keep it inside this file and then maybe we can move it into a seperate one later
    Ema,
    //Bor,
    //Indija,
    //Matija,
}
