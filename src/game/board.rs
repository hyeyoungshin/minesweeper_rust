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

//TODO: think about communication between server and players

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

type Hint = i8;
type BoardMap = HashMap<Coordinate, TileStatus>;

impl Board {
    pub fn new(h_size: u32, v_size: u32, difficulty: Difficulty) -> Board {
        let board_map = Board::initialize_board_map(h_size, v_size);

        // place mines
        Board {
            h_size,
            v_size,
            board_map,
            mine_coordinates: Board::pick_mine_coordinates(h_size, v_size, difficulty)
        }
    }
    
    // For testing only!
    // Creates a board with mines located at specified coordinates
    pub fn new_test(h_size: u32, v_size: u32, mine_coordinates: HashSet<Coordinate>) -> Board {
        let board_map = Board::initialize_board_map(h_size, v_size);

        Board{
            h_size: h_size,
            v_size: v_size,
            board_map, 
            mine_coordinates: mine_coordinates
        }        
    }

    fn initialize_board_map(h_size: u32, v_size: u32) -> BoardMap {
        let mut board_map = HashMap::new();
        // initialize all tiles
        for x in 0..h_size {
            for y in 0..v_size {
                board_map.insert(Coordinate{ x, y }, TileStatus::Hidden);
            }
        }

        board_map
    }

    pub fn pick_mine_coordinates(h_size: u32, v_size: u32, difficulty: Difficulty) -> HashSet<Coordinate> {
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

        random_coordinates
    }

    pub fn is_mine(&self, coordinate: &Coordinate) -> bool {
        self.mine_coordinates.contains(coordinate)
    }

    // Return type: Vec instead of HashSet for recursive `reveal_all`
    fn neighboring_coordinates (&self, coordinate: &Coordinate) -> Vec<Coordinate> {
        let relative_coordinates:[(i32, i32); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
        let mut neighboring_coordinates = Vec::new();

        for r_c in relative_coordinates {
            let potential_coordinate = (coordinate.x as i32 + r_c.0 , coordinate.y as i32 + r_c.1 ); // u32 as i32 is ok
            
            if self.within_bounds(&potential_coordinate) {
                neighboring_coordinates.push(Coordinate{x: potential_coordinate.0 as u32, y: potential_coordinate.1 as u32});
            }   
        }
        
        neighboring_coordinates
    }
 
    pub fn num_mines_nearby(&self, coordinate: &Coordinate) -> Hint {
        let neighbor_coordinates = self.neighboring_coordinates(coordinate);
        
        neighbor_coordinates.iter()
            .filter(|&c| self.is_mine(c))
            .count() as i8 // casting safe because it is never > 8
    }

    pub fn within_bounds(&self, potential_coordinate: &(i32, i32)) -> bool {
        potential_coordinate.0 >= 0 && potential_coordinate.0 < self.h_size as i32 && potential_coordinate.1 >= 0 && potential_coordinate.1 < self.v_size as i32
    }

    // Updates the board in place
    //  - `mut self` (instead of `&mut self`): after an update, the previous board state is gone
    //  - `player_action`` is assumed to have been validated
    pub fn update(mut self, player_action: &PlayerAction) -> Board {
        let player_coordinate = player_action.coordinate;

        match player_action.action {
            Action::Reveal => self.reveal(&player_coordinate),
            Action::Flag => { self.board_map.insert(player_coordinate, TileStatus::Flagged); },
            Action::Unflag => { self.board_map.insert(player_coordinate, TileStatus::Hidden); }
        }

        self
    }


    fn reveal_all(&mut self, neighbors: &[Coordinate]) {
        match neighbors {
            [] => (),
            [head, tail @..] => {
                self.reveal(head);
                self.reveal_all(tail);
            }
        }
    }

    fn reveal(&mut self, coordinate: &Coordinate) {
        let current_tile = self.board_map.get(coordinate).unwrap();
        match current_tile  {
            TileStatus::Hidden => {
                if self.is_mine(coordinate) {   
                    self.board_map.insert(*coordinate, TileStatus::Revealed(Tile::Mine));

                } else {
                    let hint = self.num_mines_nearby(coordinate);
                    self.board_map.insert(*coordinate, TileStatus::Revealed(Tile::Hint(hint)));
                    
                    if hint == 0 {
                        let neighbors = self.neighboring_coordinates(coordinate);
                        self.reveal_all(&neighbors);
                    } 
                }
            },
            // revealing a tile that's not hidden does not do anything
            _ => ()
        }
    }

    pub fn validate_size(h_size: i32, v_size: i32) -> Result<(u32, u32), ValidationError> {
        if h_size > MAX_SIZE as i32 && v_size > MAX_SIZE as i32 {
            Err(ValidationError::MaxExceeded)
        } else if h_size < 0 && v_size < 0 {
            Err(ValidationError::NegativeSize)
        } else {
            Ok((h_size as u32, v_size as u32))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*; // bring all of the items belonging to the tests module’s parent into scope

    fn create_3x3() -> Board {
        let mine_coordinate = Coordinate{ x: 0, y: 0 };
        
        return Board::new_test(3, 3, HashSet::from([mine_coordinate]))
    }

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
        let test_coordinate: Coordinate = Coordinate{x: 0, y: 0};
        let mine_coordinate: HashSet<Coordinate> = HashSet::from([test_coordinate]);
        let test_board: Board = Board::new_test(2, 2, mine_coordinate); // ownership of mine_coordinate moved here

        let updated_board = test_board.update(&PlayerAction{coordinate: test_coordinate, action: Action::Flag});
        let updated_tile_status = updated_board.board_map.get(&test_coordinate);

        assert_eq!(*updated_tile_status.unwrap(), TileStatus::Flagged)
    }

    #[test]
    fn test_reveal() {
        let test_coordinate: Coordinate = Coordinate{x: 0, y: 0};
        let mine_coordinate: HashSet<Coordinate> = HashSet::from([test_coordinate]);
        let player_coordinate = Coordinate{ x: 0, y: 2 };
        let mut test_board: Board = Board::new_test(3, 3, mine_coordinate); // ownership of mine_coordinate moved here

        test_board.reveal(&player_coordinate);
        assert_eq!(test_board.board_map.get(&player_coordinate).unwrap(), &TileStatus::Revealed(Tile::Hint(0)))
    }

    #[test]
    fn test_reveal_neighbor() {
        let test_coordinate: Coordinate = Coordinate{x: 0, y: 0};
        let mine_coordinate: HashSet<Coordinate> = HashSet::from([test_coordinate]);
        let mut test_board: Board = Board::new_test(2, 2, mine_coordinate); // ownership of mine_coordinate moved here

        test_board.reveal(&test_coordinate);
        assert_eq!(test_board.board_map.get(&test_coordinate).unwrap(), &TileStatus::Revealed(Tile::Mine))
    }

    #[test]
    fn test_neighboring_coordinates() {
        let test_board = create_3x3();
        let player_coordinate = Coordinate{ x: 0, y: 2 };

        assert_eq!(test_board.neighboring_coordinates(&player_coordinate).len(), 3)
    }
    
    #[test]
    fn test_reveal_0_reveal_neighbor() {
        let test_board = create_3x3();
        let player_coordinate = Coordinate{ x: 0, y: 2 };
        // reveal (0,2)
        let updated_board = test_board.update(&PlayerAction{ coordinate: player_coordinate, action: Action::Reveal });
        // (0,2) == Revealed(0)
        let neighbor_coordinate = Coordinate{ x: 0, y: 1 };
        assert_eq!(updated_board.board_map.get(&neighbor_coordinate).unwrap(), &TileStatus::Revealed(Tile::Hint(1)));
        assert_eq!(updated_board.board_map.get(&Coordinate{ x: 2, y: 2 }).unwrap(), &TileStatus::Revealed(Tile::Hint(0)));
        assert_eq!(updated_board.board_map.get(&Coordinate{ x: 1, y: 2 }).unwrap(), &TileStatus::Revealed(Tile::Hint(0)));
        assert_eq!(updated_board.board_map.get(&Coordinate{ x: 0, y: 2 }).unwrap(), &TileStatus::Revealed(Tile::Hint(0)));
        assert_eq!(updated_board.board_map.get(&Coordinate{ x: 0, y: 1 }).unwrap(), &TileStatus::Revealed(Tile::Hint(1)));
    }
}
