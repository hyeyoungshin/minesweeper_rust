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
    pub turn_order: Vec<PlayerId>, // [1,3,5,4] where 1, 3, 5, 4 are player ids
    pub current_turn: usize,       // index of turn_order
}

#[derive(PartialEq, Debug)]
pub enum GameStatus {
    Continue,
    Over,
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
    // pub fn add_player(&mut self, player: Player) {
    //     self.turn_order.push(player.id);
    //     self.players.insert(player.id, player);
    // }
    pub fn add_player(self, player: Player) -> Game {
        let mut new_turn_order = self.turn_order;
        new_turn_order.push(player.id);

        Game {
            board: self.board,
            players: self.players.update(player.id, player),
            status: self.status, 
            turn_order: new_turn_order,
            current_turn: self.current_turn,
        }
    }

    // this way I can chain add_player to game
    // for example,
    //   let mut game = Game::new(10, 10, Difficulty::Medium)
    //     .add_player(Player::new("charlie")
    //     .add_player(Player::new("hyeyoung")
    //     .add_player(Player::new("william");
    pub fn add_player_by_name(self, player_name: &str) -> Game{
        let player = Player::new(player_name.to_string());
        
        let mut new_turn_order = self.turn_order;
        new_turn_order.push(player.id);

        Game {
            board: self.board,
            players: self.players.update(player.id, player),
            status: self.status, 
            turn_order: new_turn_order,
            current_turn: self.current_turn,
        }
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
    pub fn update_status(&self, player_action: &PlayerAction, board: &Board) -> (GameStatus, i32) {
        let current_tile = board.get_tile(&player_action.coordinate);
        
        match current_tile {
            TileStatus::Revealed(Tile::Mine) => (GameStatus::Over, -10),
            TileStatus::Revealed(Tile::Hint(_)) => (GameStatus::Over, 1), //TODO: revealed more than 1 tile, 3
            TileStatus::Flagged(_) => (GameStatus::Continue, 10),
            _ => panic!("current_tile should not be hidden")
            // _ => match self.check_win(board) {
            //     true => (GameStatus::Over, 5), // Win
            //     false => (GameStatus::Continue, 1),
            // }
        }
    }

    // TODO: implement this
    pub fn get_winners(&self) -> Vec<&Player> {
        if self.players.is_empty() {
          return Vec::new();
        }
        
        let max_score = self.players.values()
            .map(|p| p.points)
            .max()
            .unwrap();
        
        self.players.values()
          .filter(|p| p.points == max_score)
          .collect()
    }

    // Check the game winning condition
    // Win condition:
    // - All tiles that don't contain mines have been revealed
    // - You can leave mines unflagged and still win
    // Lose condition:
    // - You reveal a tile with a mine (game over)
    fn check_win(&self, board: &Board) -> bool {
        board.iter().all(|(coordinate, tile_status)| {
            board.is_mine(coordinate) || matches!(tile_status, TileStatus::Revealed(Tile::Hint(_)))
        })
    }

    // Updates board_map and GameStatus
    pub fn update(&self, player_action: &PlayerAction) -> Game {
        let updated_board = self.board.update(player_action);
        let (updated_status, points) = Game::update_status(self, player_action, &updated_board);
        let current_player = self.get_player(&player_action.player_id);

        let updated_players = self.players.update(player_action.player_id, current_player.add_points(points));

        Game {
            board: updated_board,
            players: updated_players,
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
        let player_1 = Player::new_with_id(1, "hyeyoung");
        // let updated_game = game.add_player(player_1);
        game = game.add_player(player_1);

        assert_eq!(game.get_player(&1).id, 1);
    }
    
    #[test]
    fn add_players() {
        let mut game = Game::new(3, 3, Difficulty::Easy);
        let player_1 = Player::new_with_id(1, "hyeyoung");
        let player_2= Player::new_with_id(2,"charlie");
        game = game.add_player(player_1);
        game = game.add_player(player_2);

        assert_eq!(game.get_player(&2).id, 2);
    }

    #[test]
    fn get_winner() {
        let game = Game::new(3, 3, Difficulty::Easy);
        let player_1 = Player::new_with_id(1, "hyeyoung");
        let player_2 = Player::new_with_id(2, "charlie");
        let player_3 = Player::new_with_id(3, "william");
        let up1 = player_1.add_points(-1);
        let up2 = player_2.add_points(30);
        let up3 = player_3.add_points(25);
        let ugame = game.add_player(up1).add_player(up2).add_player(up3);

        assert_eq!(ugame.get_winners().len(), 1);
    }

    #[test]
    fn get_winners() {
        let game = Game::new(3, 3, Difficulty::Easy);
        let player_1 = Player::new_with_id(1, "hyeyoung");
        let player_2 = Player::new_with_id(2, "charlie");
        let player_3 = Player::new_with_id(3, "william");
        let player_4 = Player::new_with_id(4, "michael");
        let up1 = player_1.add_points(-1);
        let up2 = player_2.add_points(30); // winner
        let up3 = player_3.add_points(25);
        let up4 = player_4.add_points(30); // winner
        let ugame = game
          .add_player(up1)
          .add_player(up2)
          .add_player(up3)
          .add_player(up4);

        assert_eq!(ugame.get_winners().len(), 2);
    }
}
