pub mod board; 

use board::RefBoard;
use board::RefTile;
use board::Coordinate;
use board::TileStatus;
use crate::parse::ValidationError;

#[derive(Clone)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

pub struct Game {
    pub ref_board: RefBoard,    
    pub status: GameStatus,
}

#[derive(PartialEq, Debug)]
pub enum GameStatus {
    Continue,
    Error,
    Over, //TODO: Lose or Win?
}

// pub enum Error {
//     SizeInvalid,
//     CoordinateInvalid,
//     ActionInvalid
// }

pub struct PlayerAction {
    pub coordinate: Coordinate,
    pub action: Action, 
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Action{
    Reveal, 
    Flag, 
    Unflag
}

impl Game {
    pub fn make_move(&self, a: &PlayerAction) -> Game {
        let mut new_board_map = self.ref_board.board_map.clone();

        let current_tile = self.ref_board.board_map.get(&a.coordinate).expect("tile must exist");

        let new_game_status = 
            match (a.action, current_tile.has_mine) {
                (Action::Reveal, true) => GameStatus::Over,
                (Action::Reveal, false) => GameStatus::Continue,
                (Action::Flag,_) => GameStatus::Continue,
                (Action::Unflag, _) => GameStatus::Continue
            };
            
        let new_tile_status = match a.action {
            Action::Reveal => TileStatus::Revealed,
            Action::Flag => TileStatus::Flagged,
            Action::Unflag => TileStatus::Hidden
        };

        new_board_map.insert(a.coordinate, RefTile{has_mine: current_tile.has_mine, status: new_tile_status});
        
        Game {
            ref_board: RefBoard{
                x_size: self.ref_board.x_size, 
                y_size: self.ref_board.y_size, 
                board_map: new_board_map},
            status: new_game_status
        }
    }

    // This function validates player's chosen coordinate 
    pub fn validate_coordinate(&self, coordinate: &Coordinate) -> Result<Coordinate, ValidationError> {
        if self.ref_board.within_bounds(&(coordinate.x as i32, coordinate.y as i32)) {
            let tile = self.ref_board.board_map.get(coordinate).unwrap();

            match tile.status {
                TileStatus::Revealed => Err(ValidationError::TileRevealed),
                _ => Ok(*coordinate)
            }
        } else {
           Err(ValidationError::OutOfBounds)
        }
    }

    // pub fn validate_coordinate(&self, coordinate: &Coordinate) -> Option<Coordinate> {
    //     let tile = self.ref_board.board_map.get(coordinate).unwrap();
    //     match tile.status {
    //         TileStatus::Revealed => None, // the tile is already revealed, no more valid action available
    //         _ => {
    //             if coordinate.x < self.ref_board.x_size && coordinate.y < self.ref_board.y_size {
    //                 Some(*coordinate)
    //             } else {
    //                 None // coordinate out of bounds
    //             } 
    //         }
    //     }
    // }

    // This function validates player's chosen action for the tile at the coordinate
    pub fn validate_action(&self, action: Action, coordinate: &Coordinate) -> Option<Action> {
        let ref_tile = self.ref_board.board_map.get(coordinate).unwrap();

        match (ref_tile.status, action) {
            (TileStatus::Hidden, Action::Flag | Action::Reveal) => Some(action),
            (TileStatus::Flagged, Action::Unflag) => Some(action),
            _ => None
        }
    }
}

pub fn new_game(board_size_x: u32, board_size_y: u32, d: Difficulty) -> Game {
    let new_ref_board = RefBoard::new(board_size_x, board_size_y);
    
    Game {
        ref_board: new_ref_board.plant_mines(d),
        status: GameStatus::Continue
    }
}

// This function picks an Action randomly. Used for automatic play.
pub fn random_action() -> Action {
    use rand::Rng;
    match rand::thread_rng().gen_range(0..3) {
        0 => Action::Reveal,
        1 => Action::Flag,
        _ => Action::Unflag,
    }
}
