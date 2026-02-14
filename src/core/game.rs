use crate::core::player::{Player, PlayerId, PlayerAction, Action};
use crate::core::board::{Board, Tile, TileStatus};

use im::HashMap;

pub const EASY: f32 = 0.12;
pub const MEDIUM: f32 = 0.15;
pub const HARD: f32 = 0.2;

pub struct Game {
    pub board: Board,
    pub players: HashMap<PlayerId, Player>,
    pub status: GameStatus
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
        Game {
            board: self.board,
            players: self.players.update(player.id, player),
            status: self.status, 
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

        Game {
            board: self.board,
            players: self.players.update(player.id, player),
            status: self.status,
        }
    }

    pub fn get_player(&self, player_id: &PlayerId) -> &Player {
        self.players.get(player_id).unwrap_or_else(|| panic!("no player with id: {player_id} found"))
    }

    // pub fn current_player(&self) -> &Player {
    //     let id = self.turn_order[self.current_turn];
    //     self.get_player(&id)
    // }

    // TODO: reimplement game status update logic
    pub fn update_status(board: &Board) -> GameStatus {
        let game_over = board.iter().all(|(_, tile_status)| {
            matches!(tile_status, TileStatus::Revealed(_)) || matches!(tile_status, TileStatus::Flagged(_))
        });

        match game_over {
            true => GameStatus::Over,
            false => GameStatus::Continue,
        }
    }

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

    fn calculate_points(player_action: &PlayerAction, board: &Board) -> i32 {
        match player_action.action {
            Action::Reveal => {
                match board.get_tile(&player_action.coordinate) {
                    TileStatus::Revealed(Tile::Hint(n)) => match n {
                        0 => 3,
                        _ => 1
                    },
                    TileStatus::Revealed(Tile::Mine) => -10,
                    _ => 0 // panic!("tile should have been revealed!")
                }
            },
            Action::Flag => {
                if board.is_mine(&player_action.coordinate) {
                    2
                } else {
                    -1
                }
            }
        }
    }

    fn award_points(&self, player_action: &PlayerAction, points: i32) -> HashMap<PlayerId, Player> {
        let updated_player = self.get_player(&player_action.player_id).add_points(points);
        self.players.update(player_action.player_id, updated_player)

    }

    // Updates board_map and GameStatus
    pub fn update(&self, player_action: &PlayerAction) -> Game {
        // 1. update board
        let updated_board = self.board.update(player_action);

        // 2. calculate points based on updated_board
        let points = Self::calculate_points(player_action, &updated_board);

        // 3. award points
        let updated_players = self.award_points(player_action, points);
        
        // 4. update game status
        let updated_status = Game::update_status(&updated_board);
        
        Game {
            board: updated_board,
            players: updated_players,
            status: updated_status,
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
