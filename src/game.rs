pub mod board; 

use board::Board;
use board::Coordinate;
use board::TileStatus;
use board::*;
use crate::text_ui::ValidationError;

use std::collections::HashSet;

pub struct Game {
    pub board: Board,    
    pub status: GameStatus,
}

#[derive(PartialEq, Debug)]
pub enum GameStatus {
    Continue,
    Over,
    Win
}

#[derive(Clone)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard
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
    pub fn new(board_size_x: u32, board_size_y: u32, difficulty: Difficulty) -> Game {
        let new_board = Board::new(board_size_x, board_size_y, difficulty);
        
        Game {
            board: new_board,
            status: GameStatus::Continue
        }
    }

    // *For testing only
    // Start a game where mine locations are predetermined by you
    pub fn new_test(board_size_x: u32, board_size_y: u32, mine_coordinates: HashSet<Coordinate>) -> Game {
        let test_board = Board::new_test(board_size_x, board_size_y, mine_coordinates);
        
        Game {
            board: test_board,
            status: GameStatus::Continue
        }
    }

    // Updates the game status based on the following logic
    // If a mine is revealed, game over
    // If not, then check whether all the non-mine tiles are revealed by calling check_win
    //     If so, game win
    //     If not, game continues
    pub fn update_status(player_action: &PlayerAction, board: &Board) -> GameStatus {
        let current_tile = board.board_map.get(&player_action.coordinate)
            .expect("Coordinate should be valid and board_map should contain all valid coordinates");
        
        match current_tile {
            TileStatus::Revealed(Tile::Mine) => GameStatus::Over,
            _ => match Game::check_win(board) {
                true => GameStatus::Win,
                false => GameStatus::Continue
            }
        }
    }

    // Check the game winning condition
    // Win condition:
    // - All tiles that don't contain mines have been revealed
    // - You can leave mines unflagged and still win
    // Lose condition:
    // - You reveal a tile with a mine (game over)
    fn check_win(board: &Board) -> bool {
        board.board_map.iter().all(|(coordinate, tile_status)| {
            board.is_mine(coordinate) || matches!(tile_status, TileStatus::Revealed(Tile::Hint(_)))
        })
    }

    // Updates board_map and GameStatus
    pub fn update(self, player_action: &PlayerAction) -> Game {
        let updated_board = self.board.update(player_action); // mut self ver: no other program has access to current_board
                                                                             // &mut self ver: 
        
        let updated_status = Game::update_status(player_action, &updated_board);

        Game {
            board: updated_board,
            status: updated_status
        }
    }
    
    // This function validates player's chosen action for the tile at the coordinate
    pub fn validate_action(&self, action: Action, coordinate: &Coordinate) -> Option<Action> {
        let tile_status = self.board.board_map.get(coordinate)
            .expect("Coordinate should be valid and board_map should contain all valid coordinates");

        match (tile_status, action) {
            (TileStatus::Hidden, Action::Flag | Action::Reveal) => Some(action),
            (TileStatus::Flagged, Action::Unflag) => Some(action),
            _ => None
        }
    }

}

impl Action {
    // This function picks an Action randomly. Used for automatic play.
    pub fn random_action() -> Action {
        use rand::Rng;
        match rand::thread_rng().gen_range(0..3) {
            0 => Action::Reveal,
            1 => Action::Flag,
            _ => Action::Unflag,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*; // bring all of the items belonging to the tests module’s parent into scope

    #[test]
    fn check_win_test() {
        let mine_coordinates = HashSet::from([Coordinate{ x: 0, y: 0}, Coordinate{ x: 1, y: 1}]);

        let mut test = Game::new_test(2,2, mine_coordinates);

        test = test.update(&PlayerAction{ coordinate: Coordinate{x: 0, y: 1}, action: Action::Reveal });
        test = test.update(&PlayerAction{ coordinate: Coordinate{x: 1, y: 0}, action: Action::Reveal });

        assert_eq!(test.status, GameStatus::Win);
    }
}
