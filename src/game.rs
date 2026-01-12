pub mod board; 

use board::RefBoard;
use board::RefTile;
use board::Coordinate;
use board::TileStatus;
use crate::text_ui::ValidationError;

use std::collections::HashSet;
use std::collections::HashMap;

pub struct Game {
    pub ref_board: RefBoard,    
    pub status: GameStatus,
}

#[derive(PartialEq, Debug)]
pub enum GameStatus {
    Continue,
    Over,
    Win
}

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
    // Updates board_map and GameStatus
    pub fn update(&self, a: &PlayerAction) -> Game {
        // let mut board_map_clone = self.ref_board.board_map.clone();

        let updated_board = self.ref_board.update(a);
        let updated_status = self.update_status(&updated_board);

        // let current_tile = self.ref_board.board_map.get(&a.coordinate).unwrap();

        // let new_tile_status = match a.action {
        //     Action::Reveal => TileStatus::Revealed,
        //     Action::Flag => TileStatus::Flagged,
        //     Action::Unflag => TileStatus::Hidden
        // };

        // board_map_clone.insert(a.coordinate, RefTile{has_mine: current_tile.has_mine, status: new_tile_status});

        // // TODO: update_status checks whether win or lose
        // // We might be able to make this faster by adding counter or using im HashMap with persistent memory
        // let new_game_status: GameStatus = 
        //     match (a.action, current_tile.has_mine) {
        //         (Action::Reveal, true) => GameStatus::Over,
        //         _ => {
        //             if self.check_win(&board_map_clone) {
        //                 GameStatus::Win
        //             } else {
        //                 GameStatus::Continue
        //             }
        //         }
        //     };

        Game {
            ref_board: updated_board,
            status: updated_status
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

    fn update_status(&self, board_map: &HashMap<Coordinate, RefTile>) -> GameStatus {

    }

    // Check the game winning condition
    // Win condition:
    // - All tiles that don't contain mines have been revealed
    // - You can leave mines unflagged and still win
    // Lose condition:
    // - You reveal a tile with a mine (game over)
    fn check_win(&self, new_board_map: &HashMap<Coordinate, RefTile>) -> bool {
        new_board_map.iter().all(|(_, tile)| {
            tile.has_mine || tile.status == TileStatus::Revealed
        })

        // My inferior implementation 
        // all_coordinates(self.ref_board.x_size, self.ref_board.y_size)
        //     .into_iter()
        //     .all(|coordinate| {
        //             let ref_tile = new_board_map.get(&coordinate).unwrap();
        //             if ref_tile.has_mine {
        //                 ref_tile.status == TileStatus::Flagged || ref_tile.status == TileStatus::Hidden
        //             } else {
        //                 ref_tile.status == TileStatus::Revealed
        //             }
        //         }
        //     )
    }

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

pub fn new_game(board_size_x: u32, board_size_y: u32, num_mines: u32) -> Game {
    let new_ref_board = RefBoard::new(board_size_x, board_size_y);
    
    Game {
        ref_board: new_ref_board.place_mines(num_mines),
        status: GameStatus::Continue
    }
}

pub fn test_game(board_size_x: u32, board_size_y: u32, mine_coordinates: HashSet<Coordinate>) -> Game {
    let new_ref_board = RefBoard::new(board_size_x, board_size_y);
    
    Game {
        ref_board: new_ref_board.place_mines_at(mine_coordinates),
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


#[cfg(test)]
mod tests {
    use super::*; // bring all of the items belonging to the tests module’s parent into scope

    #[test]
    fn check_win_test() {
        let mut mine_coordinates = HashSet::new();
        mine_coordinates.insert(Coordinate{ x: 0, y: 0});
        mine_coordinates.insert(Coordinate{ x: 1, y: 1});
        
        let mut test = test_game(2,2, mine_coordinates);

        test = test.update(&PlayerAction{ coordinate: Coordinate{x: 0, y: 1}, action: Action::Reveal });
        test = test.update(&PlayerAction{ coordinate: Coordinate{x: 1, y: 0}, action: Action::Reveal });

        assert_eq!(test.status, GameStatus::Win);
    }
}
