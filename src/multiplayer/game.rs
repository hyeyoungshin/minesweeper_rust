use crate::core::board::Board;
use crate::core::player::{Player, PlayerID};
use crate::single_player::game::Difficulty;

use im::HashMap;
pub struct MultiplayerGame {
    pub board: Board,
    pub players: HashMap<PlayerID, Player>,
    pub status: GameStatus,
}

pub enum GameStatus {
    Continue,
    Over,
    Win(PlayerID)
}

impl MultiplayerGame {
    pub fn new(h_size: u32, v_size: u32, difficulty: Difficulty) -> Self {
        let board = Board::new(h_size, v_size, difficulty);
        let players = HashMap::new();

        MultiplayerGame {
            board,
            players,
            status: GameStatus::Continue,
        }
    }

    pub fn add_player(self, player: &Player) -> Self {
        MultiplayerGame {
            board: self.board,
            players: self.players.update(player.id, player.clone()),
            status: self.status,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Mul;

    use super::*; // bring all of the items belonging to the tests module’s parent into scope

    #[test]
    fn add_player() {
        let new_game = MultiplayerGame::new(3, 3, Difficulty::Easy);
        let player_1 = Player::new("hyeyoung".to_string());
        let updated_game = new_game.add_player(&player_1);

        assert_eq!(updated_game.players.get(&1).unwrap(), &player_1);
    }
    
    #[test]
    fn add_players() {
        let new_game = MultiplayerGame::new(3, 3, Difficulty::Easy);
        let player_1 = Player::new("hyeyoung".to_string());
        let player_2= Player::new("charlie".to_string());
        let updated_game = new_game.add_player(&player_1);
        let updated_game = updated_game.add_player(&player_2);

        assert_eq!(updated_game.players.get(&2).unwrap(), &player_2);
    }
}
