use crate::game::board::Coordinate;

pub struct PlayerAction {
    pub player: Player,
    pub coordinate: Coordinate,
    pub action: Action, 
}

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    pub id: String,
    pub points: i8
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Action{
    Reveal, // Revealing all in the hint = 0 case is always 3. Revealing a hint tile is 1.
    Flag, // If flagged a non-mine tile, it reveals. In this case, even if the tile has hint = 0, it does not reveal all of its neighbors. The player gets a penalty point -1.
}

impl Player {
    pub fn new(id: String, points: i8) -> Self {
        Player { id, points }
    }
}