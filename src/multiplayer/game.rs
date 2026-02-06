use crate::core::board::Board;
use crate::core::player::{Player, PlayerID};
use crate::single_player::game::{Game, Difficulty};

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
        let single_mode = Game::new(h_size, v_size, difficulty);
        let players = HashMap::new();

        MultiplayerGame {
            board: single_mode.board,
            players,
            status: GameStatus::Continue,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.insert(player.id, player);
    }
}