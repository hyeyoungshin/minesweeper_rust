use crate::core::board::{Board, Coordinate, Tile, TileStatus};
use crate::core::player::{Player, Action, PlayerAction};

use crate::single_player::text_ui::*;

use std::collections::HashSet;

pub const EASY: f32 = 0.12;
pub const MEDIUM: f32 = 0.15;
pub const HARD: f32 = 0.2;

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
        let current_tile = board.get_tile(&player_action.coordinate);
        
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
        board.iter().all(|(coordinate, tile_status)| {
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
}


#[cfg(test)]
mod tests {
    use super::*; // bring all of the items belonging to the tests module’s parent into scope

    #[test]
    fn check_win_test() {
        let mine_coordinates = HashSet::from([Coordinate{ x: 0, y: 0}, Coordinate{ x: 1, y: 1}]);

        let mut test = Game::new_test(2,2, mine_coordinates);
        let player = Player::new("hyeyoung".to_string());

        // TODO: player.clone() is annoying. Find a way to avoid it?
        test = test.update(&PlayerAction{ player_id: player.id, coordinate: Coordinate{x: 0, y: 1}, action: Action::Reveal });
        test = test.update(&PlayerAction{ player_id: player.id, coordinate: Coordinate{x: 1, y: 0}, action: Action::Reveal });

        assert_eq!(test.status, GameStatus::Win);
    }
}
