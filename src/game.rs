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
    pub fn update(&self, player_action: &PlayerAction) -> Game {
        let updated_board = self.ref_board.update(player_action);
        let updated_status = update_status(player_action, &updated_board.board_map);

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

fn test_game(board_size_x: u32, board_size_y: u32, mine_coordinates: &HashSet<Coordinate>) -> Game {
    let new_ref_board = RefBoard::new(board_size_x, board_size_y);
    
    Game {
        ref_board: new_ref_board.place_mines_at(mine_coordinates),
        status: GameStatus::Continue
    }
}

// Updates the game status based on the following logic
// If a mine is revealed, game over
// If not, then check whether all the non-mine tiles are revealed by calling check_win
//     If so, game win
//     If not, game continues
fn update_status(player_action: &PlayerAction, board_map: &HashMap<Coordinate, RefTile>) -> GameStatus {
        let updated_tile = board_map.get(&player_action.coordinate).unwrap();

        if updated_tile.has_mine && updated_tile.status == TileStatus::Revealed {
            GameStatus::Over
        } else {
            if check_win(board_map) {
                GameStatus::Win
            } else {
                GameStatus::Continue
            }
        }
}

// Check the game winning condition
// Win condition:
// - All tiles that don't contain mines have been revealed
// - You can leave mines unflagged and still win
// Lose condition:
// - You reveal a tile with a mine (game over)
fn check_win(new_board_map: &HashMap<Coordinate, RefTile>) -> bool {
    new_board_map.iter().all(|(_, tile)| {
        tile.has_mine || tile.status == TileStatus::Revealed
    })
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
        let mut mine_coordinates = HashSet::from([Coordinate{ x: 0, y: 0}, Coordinate{ x: 1, y: 1}]);

        let mut test = test_game(2,2, &mine_coordinates);

        test = test.update(&PlayerAction{ coordinate: Coordinate{x: 0, y: 1}, action: Action::Reveal });
        test = test.update(&PlayerAction{ coordinate: Coordinate{x: 1, y: 0}, action: Action::Reveal });

        assert_eq!(test.status, GameStatus::Win);
    }
}
