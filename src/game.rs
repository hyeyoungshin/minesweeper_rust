pub mod board; 

use board::RefBoard;
use board::RefTile;
use board::Coordinate;
use board::TileStatus;

#[derive(Clone)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

pub struct Game {
    pub ref_board: RefBoard,    
    pub status: GameStatus,
}

#[derive(PartialEq, Debug)]
pub enum GameStatus {
    Continue,
    Over,
    Error
}

pub struct PlayerAction {
    pub coordinate: Coordinate,
    pub action: Action, 
}

#[derive(Debug, PartialEq)]
pub enum Action{
    Reveal, 
    Flag, 
    Unflag
}

impl Game {
    pub fn make_move(&self, a: &PlayerAction) -> Game {
        let mut new_board_map = self.ref_board.board_map.clone();

        let current_tile = self.ref_board.board_map.get(&a.coordinate).expect("tile must exist");

        let new_game_status = 
            if is_valid_action(&current_tile.status, &a.action) {
                match a.action {
                    Action::Reveal => if current_tile.has_mine {GameStatus::Over} else {GameStatus::Continue},
                    Action::Flag => GameStatus::Continue,
                    Action::Unflag => GameStatus::Continue
                }
            } else {
                GameStatus::Error
            };

        let new_tile_status = match a.action {
            Action::Reveal => TileStatus::Revealed,
            Action::Flag => TileStatus::Flagged,
            Action::Unflag => TileStatus::Hidden
        };
    
        
        new_board_map.insert(a.coordinate, RefTile{has_mine: current_tile.has_mine, status: new_tile_status});
        
        Game {
            ref_board: RefBoard{
                x_size: self.ref_board.x_size, 
                y_size: self.ref_board.y_size, 
                board_map: new_board_map},
            status: new_game_status
        }
    }

    pub fn is_valid_coordinate(&self, coordinate: &Coordinate) -> bool {
        coordinate.x < self.ref_board.x_size && coordinate.y < self.ref_board.y_size
        // if coordinate.x < self.ref_board.x_size && coordinate.y < self.ref_board.y_size {
        //     Some(coordinate) // lifetime problem if return type is Option<&Coordinate>
        // } else {
        //     None
        // }
    }
    
}

pub fn new_game(board_size_x: u32, board_size_y: u32, d: Difficulty) -> Game {
    let new_ref_board = RefBoard::new(board_size_x, board_size_y);
    
    Game {
        ref_board: new_ref_board.plant_mines(d),
        status: GameStatus::Continue
    }
}

fn is_valid_action(t: &TileStatus, a: &Action) -> bool {
    match t {
        TileStatus::Hidden => *a == Action::Flag || *a == Action::Reveal,
        TileStatus::Flagged => *a == Action::Unflag,
        TileStatus::Revealed => false
    }
}

// make it a method or helper function?
// pub fn is_valid_coordinate(x_size: u32, y_size: u32, coordinate: &Coordinate) -> bool {
//     coordinate.x < x_size && coordinate.y < y_size
// }

pub fn random_action() -> Action {
    use rand::Rng;
    match rand::thread_rng().gen_range(0..3) {
        0 => Action::Reveal,
        1 => Action::Flag,
        _ => Action::Unflag,
    }
}

