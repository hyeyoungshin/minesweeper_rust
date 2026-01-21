use std::collections::HashMap;
use std::collections::HashSet;


use crate::game::Difficulty;
use crate::game::PlayerAction;
use crate::game::Action;
use crate::text_ui::ValidationError;


// Board's vertical and horizontal max size 
// It is set so that we can convert u32 to i32 safely during coordinate validation
const MAX_SIZE: u32 = i32::MAX as u32; // 2147483647 

pub const EASY: f32 = 0.12;
pub const MEDIUM: f32 = 0.15;
pub const HARD: f32 = 0.2;

pub struct Board { 
    pub h_size: u32,  // horizontal size (grows to right)
    pub v_size: u32,  // vertical size (grows down)
    pub board_map: HashMap<Coordinate, TileStatus>,
    mine_coordinates: HashSet<Coordinate>
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileStatus {
    Hidden,
    Flagged,
    Revealed(Tile)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Hint(i8),
    Mine
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Coordinate { pub x: u32, pub y: u32 }

impl Board {
    pub fn new(h_size: u32, v_size: u32, difficulty: Difficulty) -> Board {
        let board_map = initialize_board_map(h_size, v_size);

        // place mines
        Board {
            h_size,
            v_size,
            board_map,
            mine_coordinates: get_mine_coordinates(h_size, v_size, difficulty)
        }
    }

    pub fn is_mine(&self, coordinate: &Coordinate) -> bool {
        self.mine_coordinates.contains(coordinate)
    }

    fn neighbor_coordinates (&self, coordinate: &Coordinate) -> Vec<Coordinate> {
        let relative_coordinates:[(i32, i32); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
        let mut neighbors_coordinates: Vec<Coordinate> = Vec::new();

        for r_c in relative_coordinates {
            let potential_coordinate = (coordinate.x as i32 + r_c.0 , coordinate.y as i32 + r_c.1 ); // u32 as i32 is ok
            
            if self.within_bounds(&potential_coordinate) {
                neighbors_coordinates.push(Coordinate{x: potential_coordinate.0 as u32, y: potential_coordinate.1 as u32});
            }   
        }
        
        neighbors_coordinates
    }
 
    pub fn num_mines_nearby(&self, coordinate: &Coordinate) -> i8 {
        let neighbor_coordinates = self.neighbor_coordinates(coordinate);
        
        neighbor_coordinates.iter()
            .filter(|&c| self.mine_coordinates.contains(c))
            .count() as i8 // casting safe because it is never > 8
    }

    pub fn within_bounds(&self, potential_coordinate: &(i32, i32)) -> bool {
        potential_coordinate.0 >= 0 && potential_coordinate.0 < self.h_size as i32 && potential_coordinate.1 >= 0 && potential_coordinate.1 < self.v_size as i32
    }

    pub fn update(self, player_action: &PlayerAction) -> Board {
        let player_coordinate = player_action.coordinate;
        let current_tile_status = self.board_map.get(&player_coordinate).unwrap();

        let updated_tile_status = match player_action.action {
            Action::Reveal => {
                if self.mine_coordinates.contains(&player_coordinate) { 
                    TileStatus::Revealed(Tile::Mine)
                } else { 
                    TileStatus::Revealed(Tile::Hint(self.num_mines_nearby(&player_coordinate) as i8))
                }
            },
            Action::Flag => TileStatus::Flagged,
            Action::Unflag => TileStatus::Hidden,
        };
        
        let mut updated_board_map = self.board_map; // Move, don't clone (Functional update)

        // Take care of hint = 0 case
        match updated_tile_status {
            TileStatus::Revealed(Tile::Hint(n)) if n == 0 => self.reveal(self.neighbor_coordinates(&player_coordinate), updated_board_map),
            _ => updated_board_map.insert(player_coordinate, updated_tile_status)
         };

        Board {
            h_size: self.h_size,
            v_size: self.v_size,
            board_map: updated_board_map,
            mine_coordinates: self.mine_coordinates
        }
    }

    // fn reveal_all(self, unrevealed_neighbors: &[Coordinate], board_map: HashMap<Coordinate, TileStatus>) -> HashMap<Coordinate, TileStatus> {
    //     match unrevealed_neighbors {
    //         [] => board_map,
    //         [head, tail @..] => {
    //             // head is &T
    //             // tail is &[T]
    //             let updated_board_map = self.reveal(head, board_map);
    //             self.reveal_all(tail, updated_board_map)
    //         }
    //     }
    // }

    pub fn reveal(&self, coordinate: &Coordinate, mut board_map: HashMap<Coordinate, TileStatus>) -> HashMap<Coordinate, TileStatus> {
        match board_map.get(coordinate).unwrap()  {
            TileStatus::Hidden => if self.mine_coordinates.contains(coordinate) 
                { 
                    board_map.insert(*coordinate, TileStatus::Revealed(Tile::Mine));
                    board_map
                } else {
                    board_map.insert(*coordinate, TileStatus::Revealed(Tile::Hint(self.num_mines_nearby(coordinate))));
                    board_map
                },
            _ => board_map
       }
    }
}


//////////////////////////////////////////////////////////////////
// Support Functions
//////////////////////////////////////////////////////////////////

fn initialize_board_map(h_size: u32, v_size: u32) -> HashMap<Coordinate, TileStatus> {
    let mut board_map = HashMap::new();
    // initialize all tiles
    for x in 0..h_size {
        for y in 0..v_size {
            board_map.insert(Coordinate{ x, y }, TileStatus::Hidden);
        }
    }

    board_map
}

// Creates a board with mines located at specified coordinates
pub fn new_test_board(h_size: u32, v_size: u32, mine_coordinates: HashSet<Coordinate>) -> Board {
    let board_map = initialize_board_map(h_size, v_size);

    Board{
        h_size: h_size,
        v_size: v_size,
        board_map, 
        mine_coordinates: mine_coordinates
    }        
}

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

    const test_coordinate: Coordinate = Coordinate{x: 0, y: 0};
    const mine_coordinate: HashSet<Coordinate> = HashSet::from([test_coordinate]);
    const test_board: Board = new_test_board(2, 2, mine_coordinate); // ownership of mine_coordinate moved here


    #[test]
    fn num_mine_easy() {
        let new_board = Board::new(2, 2, Difficulty::Easy);
        assert_eq!(new_board.mine_coordinates.len(), 1)
    }

    #[test]
    fn num_mines_hard() {
        let new_board = Board::new(5, 5, Difficulty::Hard);
        assert_eq!(new_board.mine_coordinates.len(), 5);
    }

    #[test]
    fn test_update() {
        let updated_board = test_board.update(&PlayerAction{coordinate: test_coordinate, action: Action::Flag});
        let updated_tile_status = updated_board.board_map.get(&test_coordinate);

        assert_eq!(*updated_tile_status.unwrap(), TileStatus::Flagged)
    }

    #[test]
    fn test_reveal() {
        let updated_board_map = test_board.reveal(test_coordinate, test_board.board_map);
        assert_eq!(updated_board_map.get(test_coordinate).unwrap(), TileStatus::Revealed(Tile::Mine))
    }
}