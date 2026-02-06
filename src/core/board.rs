// Board, TileStatus, reveal logic

use im::HashMap;
use std::rc::Rc;
use std::collections::HashSet;


use crate::core::player::*;

// TODO: remove dependency on single_player
use crate::single_player::game::*;
use crate::single_player::game::Difficulty;
use crate::single_player::text_ui::InvalidErr;
use crate::single_player::text_ui::BOARD_MAX_SIZE;
use crate::single_player::text_ui::BoardSize;

pub struct Board { 
    pub h_size: u32,  // horizontal size (grows to right)
    pub v_size: u32,  // vertical size (grows down)
    board_map: HashMap<Coordinate, TileStatus>,
    mine_coordinates: Rc<HashSet<Coordinate>> // Shared, immutable
}

// TODO: think about communication between server and players
// Add a representation of board for players

#[derive(Debug, Clone, PartialEq)]
pub enum TileStatus {
    Hidden,
    Flagged(PlayerID),
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
        // initializes empty hashmap
        let board_map = Board::initialize_board_map(h_size, v_size);

        // place mines
        Board {
            h_size,
            v_size,
            board_map,
            mine_coordinates: Rc::new(Board::random_mine_coordinates(h_size, v_size, difficulty)),
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
            mine_coordinates: Rc::new(mine_coordinates),
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

    pub fn get_tile(&self, coordinate: &Coordinate) -> &TileStatus {
        // order of evaluation: call-by-value (like most programming languages but Haskell)
        // instead of unwrap() or expect() use unwrap_or_else to
        // - delay evaluation (in the case of expect) 
        // - add custom panic message 
        self.board_map.get(coordinate).unwrap_or_else(|| panic!("tile should be at {:?}", coordinate))
    }

    // Provides an interface for board_map
    // - uses Iterator interface instead of HashMap iterator
    pub fn iter(&self) -> impl Iterator<Item = (&Coordinate, &TileStatus)> {
        self.board_map.iter()
    }

    pub fn random_mine_coordinates(h_size: u32, v_size: u32, difficulty: Difficulty) -> HashSet<Coordinate> {
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
 
    pub fn get_hint(&self, coordinate: &Coordinate) -> Hint {
        let neighbor_coordinates = self.neighboring_coordinates(coordinate);
        
        neighbor_coordinates.iter()
            .filter(|&c| self.is_mine(c))
            .count() as i8 // casting safe because it is never > 8
    }

    pub fn within_bounds(&self, potential_coordinate: &(i32, i32)) -> bool {
        potential_coordinate.0 >= 0 && potential_coordinate.0 < self.h_size as i32 && potential_coordinate.1 >= 0 && potential_coordinate.1 < self.v_size as i32
    }

    // Updates Board using immutable hashmap
    pub fn update(&self, player_action: &PlayerAction) -> Board {
        let updated_board_map = match player_action.action {
            Action::Reveal => self.reveal(&player_action.coordinate, self.board_map.clone()),
            Action::Flag => self.board_map.update(player_action.coordinate, TileStatus::Flagged(player_action.player_id)),
            Action::Unflag => self.board_map.update(player_action.coordinate, TileStatus::Hidden),
        };

        Board {
            h_size: self.h_size,
            v_size: self.v_size,
            board_map: updated_board_map,
            mine_coordinates: Rc::clone(&self.mine_coordinates),
        }
    }

    fn reveal_all(&self, mut hidden_neighbors: Vec<Coordinate>, board_map: BoardMap) -> BoardMap {
        match hidden_neighbors.pop() {
            None => board_map,
            Some(coord) => {
                let updated_board_map = self.reveal(&coord, board_map);
                self.reveal_all(hidden_neighbors,updated_board_map)  // O(1) allocations
            }
            // Vec is owned by this function call---not shared with anyone else
            // It gets consumed and destroyed when the function returns
            // So this is still functional
        }
    }

    // fn reveal_all(&self, hidden_neighbors: Vec<Coordinate>, board_map: BoardMap) -> BoardMap {
    //     match hidden_neighbors.as_slice() {
    //         [] => board_map,
    //         [head, tail @..] => {
    //             let updated_board_map = self.reveal(head, board_map);
    //             self.reveal_all(tail.to_vec(), updated_board_map) <--------- O(n^2) allocations by .to_vec()
    //         }
    //     }
    // }

    fn reveal(&self, coordinate: &Coordinate, board_map: BoardMap) -> BoardMap {
        let tile_status = board_map.get(coordinate)
            .unwrap_or_else(|| panic!("tile should exist at {:?}", coordinate));
        
        match tile_status {
            TileStatus::Hidden => {
                let updated_tile_status = 
                    if self.is_mine(coordinate) {
                        TileStatus::Revealed(Tile::Mine)
                    } else {
                        let hint = self.get_hint(coordinate);
                        TileStatus::Revealed(Tile::Hint(hint))
                    };

                // handle hint = 0 case
                let is_zero_hint =
                    matches!(updated_tile_status, TileStatus::Revealed(Tile::Hint(0)));

                let updated_board_map = board_map.update(*coordinate, updated_tile_status);

                if is_zero_hint {
                    let hidden_neighbors = self.neighboring_coordinates(coordinate)
                        .into_iter()
                        .filter(|c| matches!(updated_board_map.get(c), Some(TileStatus::Hidden)))
                        .collect();

                    self.reveal_all(hidden_neighbors, updated_board_map)
                } else{
                    updated_board_map
                }
            },
            _ => board_map // this case is necessary because a tile could be a hidden neighbor of two different tiles
                           // one tile could be revealed more than once in the recursive call stack
        }
    }    


    pub fn validate_size(h_size: u32, v_size: u32) -> Result<BoardSize, InvalidErr> {
        if h_size > BOARD_MAX_SIZE && v_size > BOARD_MAX_SIZE {
            Err(InvalidErr::InvalidBoardSize)
        } else {
            Ok((h_size as u32, v_size as u32))
        }
    }

    

    // // Boolean helper for internal use / assertions
    // fn is_valid_coordinate(&self, coord: &Coordinate) -> bool {
    //     self.validate_coordinate(coord).is_ok()
    // }

    pub fn print(&self) {
        for y in 0..self.v_size {
            for x in 0..self.h_size {
                match self.board_map.get(&Coordinate{ x, y }).unwrap() {
                    TileStatus::Hidden => print!("? "),
                    TileStatus::Flagged(player_id) => print!("!,by {} ", player_id),
                    TileStatus::Revealed(Tile::Hint(n)) => print!("{} ", n),
                    TileStatus::Revealed(Tile::Mine) => print!("* ")
                }
            }
            println!();
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

        let player = Player::new("hyeyoung".to_string());

        let updated_board = test_board.update(&PlayerAction{player_id: player.id, coordinate: test_coordinate, action: Action::Flag});
        let updated_tile_status = updated_board.board_map.get(&test_coordinate);

        assert_eq!(*updated_tile_status.unwrap(), TileStatus::Flagged(player.id))
    }

    #[test]
    fn test_reveal() {
        let mine_coordinate: HashSet<Coordinate> = HashSet::from([Coordinate{x: 0, y: 0}]);
        let player_coordinate = Coordinate{ x: 0, y: 1 };
        let test_board: Board = Board::new_test(3, 3, mine_coordinate);


        let updated_board = test_board.reveal(&player_coordinate, test_board.board_map.clone());
        assert_eq!(updated_board.get(&player_coordinate).unwrap(), &TileStatus::Revealed(Tile::Hint(1)))
    }

    #[test]
    fn test_reveal_all() {
        let mine_coordinate: HashSet<Coordinate> = HashSet::from([Coordinate{x: 0, y: 0}]);
        let test_board: Board = Board::new_test(3, 3, mine_coordinate);

        let updated_board = test_board.reveal(&Coordinate{x: 0, y: 2}, test_board.board_map.clone());
        assert_eq!(updated_board.get(&Coordinate{x: 0, y: 1}).unwrap(), &TileStatus::Revealed(Tile::Hint(1)))
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
        let player = Player::new("hyeyoung".to_string());
        // reveal (0,2)
        let updated_board = test_board.update(&PlayerAction{ player_id: player.id, coordinate: player_coordinate, action: Action::Reveal });
        // (0,2) == Revealed(0)
        let neighbor_coordinate = Coordinate{ x: 0, y: 1 };
        
        assert_eq!(updated_board.board_map.get(&neighbor_coordinate).unwrap(), &TileStatus::Revealed(Tile::Hint(1)));
        assert_eq!(updated_board.board_map.get(&Coordinate{ x: 2, y: 2 }).unwrap(), &TileStatus::Revealed(Tile::Hint(0)));
        assert_eq!(updated_board.board_map.get(&Coordinate{ x: 1, y: 2 }).unwrap(), &TileStatus::Revealed(Tile::Hint(0)));
        assert_eq!(updated_board.board_map.get(&Coordinate{ x: 0, y: 2 }).unwrap(), &TileStatus::Revealed(Tile::Hint(0)));
        assert_eq!(updated_board.board_map.get(&Coordinate{ x: 0, y: 1 }).unwrap(), &TileStatus::Revealed(Tile::Hint(1)));
    }
}
