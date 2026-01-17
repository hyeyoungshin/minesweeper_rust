use std::collections::HashMap;
use std::collections::HashSet;

use crate::game::Difficulty;
use crate::text_ui::ValidationError;
use crate::game::PlayerAction;

// Board's vertical and horizontal max size 
// It is set so that we can convert u32 to i32 safely during coordinate validation
const MAX_SIZE: u32 = i32::MAX as u32; // 2147483647 

pub const EASY: f32 = 0.12;
pub const MEDIUM: f32 = 0.15;
pub const HARD: f32 = 0.2;

pub struct Board { 
    pub h_size: u32,  // horizontal size (grows to right)
    pub v_size: u32,  // vertical size (grows down)
    pub board_map: HashMap<Coordinate, Tile>,
    mine_coordinates: HashSet<Coordinate>
}

// Tile presentation for players
// #[derive(Debug, Clone, PartialEq)]
// pub enum PlayerTile {
//     Hidden,
//     Flagged,
//     Hint(usize), // i8 suffices since # of mines in neighboring tiles cannot exceed 8
//     Mine
// }

// #[derive(Clone)]
// pub struct RefTile {
//     pub has_mine: bool,
//     pub status: TileStatus,
// }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Hidden,
    Flagged,
    Hint(i8),
    Mine
}

// impl Tile {
//     fn update(&self, player_action: &PlayerAction) -> Tile {
//         match &player_action.action { 
//             Action::Reveal => Tile::Revealed(hint),
//             Action::Flag => Tile::Flagged,
//             Action::Unflag => Tile::Hidden
//         };
//     }
// }

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Coordinate { pub x: u32, pub y: u32 }

impl Board {
    pub fn new(h_size: u32, v_size: u32, difficulty: Difficulty) -> Self {
        let mut board_map = HashMap::new();

        // initialize all tiles
        for x in 0..h_size {
            for y in 0..v_size {
                board_map.insert(Coordinate{ x, y }, Tile::Hidden);
            }
        }
        
        // place mines
        Board {
            h_size,
            v_size,
            board_map,
            mine_coordinates: get_mine_coordinates(h_size, v_size, difficulty)
        }
    }

    pub fn new_test(&self, coordinates: HashSet<Coordinate>) -> Self {
        let mut board_map_with_mines: HashMap<Coordinate, Tile> = HashMap::new();
        
        for coordinate in coordinates.clone() {
            board_map_with_mines.insert(coordinate, Tile::Mine);
        }

        for x in 0..self.h_size {
            for y in 0..self.v_size {
                board_map_with_mines
                    .entry(Coordinate{x, y})
                    .or_insert(Tile::Hidden);
            }
        }

        Board{
            h_size: self.h_size,
            v_size: self.v_size,
            board_map: board_map_with_mines,
            mine_coordinates: coordinates
        }
        
    }

    pub fn num_mines_nearby(&self, coordinate: &Coordinate) -> usize {
        let relative_coordinates:[(i32, i32); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

        let mut neighbors_coordinates: Vec<Coordinate> = Vec::new();
        
        for r_c in relative_coordinates {
            // TODO: add board size max so that casting here is safe
            let potential_coordinate = (coordinate.x as i32 + r_c.0 , coordinate.y as i32 + r_c.1 ); // u32 as i32 is ok
            
            if self.within_bounds(&potential_coordinate) {
                neighbors_coordinates.push(Coordinate{x: potential_coordinate.0 as u32, y: potential_coordinate.1 as u32});
            }   
        }
        
        // neighbors_coordinates.len() as i8  always returns number of neighbors!
        neighbors_coordinates.iter()
            .filter(|c| self.board_map.get(c).unwrap().has_mine)
            .count()
    }

    pub fn within_bounds(&self, potential_coordinate: &(i32, i32)) -> bool {
        potential_coordinate.0 >= 0 && potential_coordinate.0 < self.h_size as i32 && 
        potential_coordinate.1 >= 0 && potential_coordinate.1 < self.v_size as i32
    }

    pub fn update(&self, player_action: &PlayerAction) -> Self {
        let coordinate = player_action.coordinate;
        let updated_tile = self.board_map.get(&coordinate).unwrap().update(&player_action);
        let mut updated_board_map = self.board_map.clone();

        // Take care of hint = 0 case
        match updated_tile.status {
            TileStatus::Revealed(hint) if hint == 0 => self.expand(&coordinate), // or expand_board_map(&Coordinate, Board)
            _ => updated_board_map.insert(coordinate, updated_tile);
         }

        RefBoard {
            h_size: self.h_size,
            v_size: self.v_size,
            board_map: updated_board_map
        }
    }
}


//////////////////////////////////////////////////////////////////
// Support Functions
//////////////////////////////////////////////////////////////////
pub fn get_mine_coordinates(h_size: u32, v_size: u32, difficulty: Difficulty) -> HashSet<Coordinate> {
        let mut random_coordinates: HashSet<Coordinate> = HashSet::new();
        let board_size = (h_size * v_size) as f32; // To compare and multiply with floating point numbers

        let num_mines: f32 = if board_size < 5.0 {
            1.0
        } else {
            board_size * 
                match difficulty {
                    Difficulty::Easy => EASY,
                    Difficulty::Medium => MEDIUM,
                    Difficulty::Hard => HARD 
                }
        };

        let num_mines = num_mines.floor() as usize;

        use rand::Rng;
        let mut rng = rand::thread_rng();

        while random_coordinates.len() < num_mines as usize {
            random_coordinates.insert( 
                Coordinate {x: rng.gen_range(0..h_size), y: rng.gen_range(0..v_size)}
            );
        }

        // For testing only!!
        println!("mines are at: {:?}", random_coordinates);
         
        // Shuffling method (better for denser mines)
        // use rand::seq::SliceRandom;

        // let mut all_coordinates: Vec<Coordinate> = all_coordinates(self.xsize, self.ysize)
        // all_coordinates.shuffle(&mut rand::thread_rng()); // shuffle inplace

        // let mine_coordinates: Vec<Coordinate> = all_coordinates
        //     .into_iter()
        //     .take(number_of_mines)
        //     .collect();

        random_coordinates
    }

pub fn validate_board_size(h_size: i32, v_size: i32) -> Result<(u32, u32), ValidationError> {
    if h_size > MAX_SIZE as i32 && v_size > MAX_SIZE as i32 {
        Err(ValidationError::MaxExceeded)
    } else if h_size < 0 && v_size < 0 {
        Err(ValidationError::NegativeSize)
    } else {
        Ok((h_size as u32, v_size as u32))
    }
}

#[cfg(test)]
mod tests {
    use super::*; // bring all of the items belonging to the tests module’s parent into scope

    #[test]
    fn check_mines() {
        let board = Board::new(5, 5);
        let board_with_mines = board.place_mines(Difficulty::Hard);
        
        let num_mines = board_with_mines.board_map.iter().fold(0, |acc, kv| if kv.1.has_mine {acc + 1} else {acc});

        assert_eq!(num_mines, 5);
    }

    #[test]
    fn test_update() {
        let board = Board::new(2, 2);
        let mine_coordinate = HashSet::from([Coordinate{x: 0, y:0}]);
        
        let board_with_mines = board.place_mines_at(&mine_coordinate);

        let updated_board = board_with_mines.update(&PlayerAction{coordinate: Coordinate{x: 0, y: 0}, action: Action::Flag});
        let updated_tile = updated_board.board_map.get(mine_coordinate.iter().next().unwrap()).unwrap();

        assert_eq!(updated_tile.status, TileStatus::Flagged)
    }

    #[test]
    fn test_place_mine() {
        let board = Board::new(2, 2);
        
        let board_with_mines = board.place_mines(Difficulty::Easy);
        let num_mines = board_with_mines.board_map.iter().fold(0, |acc, kv| if kv.1.has_mine {acc + 1} else {acc});

        assert_eq!(num_mines, 1)
    }
}