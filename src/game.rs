use std::collections::HashMap;

use crate::board::RefBoard;
use crate::board::RefTile;
use crate::board::Coordinate;
use crate::board::TileStatus;


#[derive(Clone)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

struct Game {
    ref_board: RefBoard,    
    status: GameStatus,
}

enum GameStatus {
    Continue,
    Over,
    Error
}

struct PlayerAction {
    coordinate: Coordinate,
    action: Action, 
}

#[derive(PartialEq)]
enum Action{
    Reveal, 
    Flag, 
    Unflag
}

impl Game {
    fn new(&self, size: u32, d: Difficulty) -> Game {
        let new_ref_board = RefBoard::new(size, size);
        
        Game {
            ref_board: new_ref_board.plant_mines(&d),
            status: GameStatus::Continue
        }
    }

    fn make_move(&self, a: PlayerAction) -> Game {
        let mut new_board_map = self.ref_board.board_map.clone();

        let current_tile = self.ref_board.board_map.get(&a.coordinate).expect("tile must exist");

        let new_tile_status = match a.action {
            Action::Reveal => TileStatus::Revealed,
            Action::Flag => TileStatus::Flagged,
            Action::Unflag => TileStatus::Hidden
        };

        let new_game_status = 
            if Self::is_valid(&current_tile.status, &a.action) {
                match a.action {
                    Action::Reveal => if current_tile.has_mine {GameStatus::Over} else {GameStatus::Continue},
                    Action::Flag => GameStatus::Continue,
                    Action::Unflag => GameStatus::Continue
                }
            } else {
                GameStatus::Error
            };
        
        new_board_map.insert(a.coordinate, RefTile{has_mine: current_tile.has_mine, status: new_tile_status});
        
        Game {
            ref_board: RefBoard{
                xsize: self.ref_board.xsize, 
                ysize: self.ref_board.ysize, 
                board_map: new_board_map},
            status: new_game_status
        }
    }

    fn is_valid(t: &TileStatus, a: &Action) -> bool {
        match t {
            TileStatus::Hidden => *a == Action::Flag || *a == Action::Reveal,
            TileStatus::Flagged => *a == Action::Unflag,
            TileStatus::Revealed => false
        }
    }
}
