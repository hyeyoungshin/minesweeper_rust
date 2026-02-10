use crate::core::player::{Player, PlayerId, PlayerAction};
use crate::core::board::{Board, Tile, TileStatus};

use im::HashMap;

pub const EASY: f32 = 0.12;
pub const MEDIUM: f32 = 0.15;
pub const HARD: f32 = 0.2;

pub struct Game {
    pub board: Board,
    pub players: HashMap<PlayerId, Player>,
    pub status: GameStatus,
    pub turn_order: Vec<PlayerId>, 
    pub current_turn: usize,
}

#[derive(PartialEq, Debug)]
pub enum GameStatus {
    Continue,
    Over,
    Win(PlayerId)
}

#[derive(Clone)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

impl Game {
    pub fn new(h_size: u32, v_size: u32, difficulty: Difficulty) -> Game {
        Game {
            board: Board::new(h_size, v_size, difficulty),
            players: HashMap::new(),
            status: GameStatus::Continue,
            turn_order: Vec::new(),
            current_turn: 0,
        }
    }
    
    // once a player is added to a game, the game owns that player
    // The Rust Guideline:  
    //   "If a function consumes data to store it, take ownership."
    // When You WOULD Borrow:
    //   Only if the caller legitimately needs the player afterwards:
    pub fn add_player(&mut self, player: Player) {
        self.turn_order.push(player.id);
        self.players.insert(player.id, player);
        
    }

    pub fn get_player(&self, player_id: &PlayerId) -> &Player {
        self.players.get(player_id).unwrap_or_else(|| panic!("no player with id: {player_id} found"))
    }

    pub fn current_player(&self) -> &Player {
        let id = self.turn_order[self.current_turn];
        self.get_player(&id)
    }

    // Updates the game status based on the following logic
    // If a mine is revealed, game over
    // If not, then check whether all the non-mine tiles are revealed by calling check_win
    //     If so, game win
    //     If not, game continues
    pub fn update_status(&self, player_action: &PlayerAction, board: &Board) -> GameStatus {
        let current_tile = board.get_tile(&player_action.coordinate);
        
        match current_tile {
            TileStatus::Revealed(Tile::Mine) => GameStatus::Over,
            _ => match self.check_win(board) {
                (true, winner) => GameStatus::Win(winner.id),
                (false, _) => GameStatus::Continue
            }
        }
    }

    // Check the game winning condition
    // Win condition:
    // - All tiles that don't contain mines have been revealed
    // - You can leave mines unflagged and still win
    // Lose condition:
    // - You reveal a tile with a mine (game over)
    fn check_win(&self, board: &Board) -> (bool, &Player) {
        let won = board.iter().all(|(coordinate, tile_status)| {
            board.is_mine(coordinate) || matches!(tile_status, TileStatus::Revealed(Tile::Hint(_)))
        });

        (won, self.current_player())
    }

    // Updates board_map and GameStatus
    pub fn update(&self, player_action: &PlayerAction) -> Game {
        let updated_board = self.board.update(player_action);
        let updated_status = Game::update_status(self, player_action, &updated_board);

        Game {
            board: updated_board,
            players: self.players.clone(),
            status: updated_status,
            turn_order: self.turn_order.clone(),
            current_turn: (self.current_turn + 1) % self.turn_order.len(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*; // bring all of the items belonging to the tests module’s parent into scope

    #[test]
    fn add_player() {
        let mut game = Game::new(3, 3, Difficulty::Easy);
        let player_1 = Player::new_with_id(1, "hyeyoung".to_string());
        // let updated_game = game.add_player(player_1);
        game.add_player(player_1);

        assert_eq!(game.get_player(&1).id, 1);
    }
    
    #[test]
    fn add_players() {
        let mut game = Game::new(3, 3, Difficulty::Easy);
        let player_1 = Player::new_with_id(1, "hyeyoung".to_string());
        let player_2= Player::new_with_id(2,"charlie".to_string());
        game.add_player(player_1);
        game.add_player(player_2);

        assert_eq!(game.get_player(&2).id, 2);
    }
}
