use std::collections::HashMap;
use std::collections::HashSet;
use crate::text_ui::ValidationError;
use crate::game::PlayerAction;
use crate::game::Action;


// Board's vertical and horizontal max size 
// It is set so that we can convert u32 to i32 safely during coordinate validation
const MAX_SIZE: u32 = i32::MAX as u32; // 2147483647 

pub struct Board<Tile> { // Design Decision: making `Tile` 
                         // 1. Parameter 
                         // 2. Trait
                         // depends on whether Board needs to interact with Tile in its implementation or not
    pub x_size: u32,  // horizontal size (grows to right)
    pub y_size: u32,  // vertical size (grows down)
    pub board_map: HashMap<Coordinate, Tile>,
}

// Tile presentation for players
#[derive(Debug, Clone, PartialEq)]
pub enum PlayerTile {
    Hidden,
    Flagged,
    Hint(usize), // i8 suffices since # of mines in neighboring tiles cannot exceed 8
    Mine
}

#[derive(Clone)]
pub struct RefTile {
    pub has_mine: bool,
    pub status: TileStatus,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileStatus {
    Hidden,
    Flagged,
    Revealed
}

impl RefTile {
    fn update(&self, player_action: &PlayerAction) -> RefTile {
        let updated_status = match &player_action.action { 
            Action::Reveal => TileStatus::Revealed,
            Action::Flag => TileStatus::Flagged,
            Action::Unflag => TileStatus::Hidden
        };

        RefTile {
            has_mine: self.has_mine,
            status: updated_status
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Coordinate { pub x: u32, pub y: u32 }

pub type RefBoard = Board<RefTile>;

impl RefBoard {
    pub fn new(x_size: u32, y_size: u32) -> Self {
        let mut board_map = HashMap::new();

        // initialize all tiles
        for x in 0..x_size {
            for y in 0..y_size {
                board_map.insert(Coordinate{ x, y }, RefTile { has_mine: false, status: TileStatus::Hidden });
            }
        }
        
        // place mines
        Board {
            x_size: x_size,
            y_size: y_size,
            board_map,
        }
    }

    pub fn place_mines_at(&self, coordinates: &HashSet<Coordinate>) -> Self {
        let mut board_map_with_mines: HashMap<Coordinate, RefTile> = HashMap::new();
        
        for coordinate in coordinates.clone() {
            board_map_with_mines.insert(coordinate, RefTile{has_mine: true, status: TileStatus::Hidden});
        }

        for x in 0..self.x_size {
            for y in 0..self.y_size {
                board_map_with_mines
                    .entry(Coordinate{x, y})
                    .or_insert(RefTile{has_mine: false, status: TileStatus::Hidden});
            }
        }

        Board{
            x_size: self.x_size,
            y_size: self.y_size,
            board_map: board_map_with_mines,
        }
        
    }

    pub fn place_mines(&self, num_mines: u32) -> RefBoard {
        let mut random_coordinates: HashSet<Coordinate> = HashSet::new();

        while random_coordinates.len() < num_mines as usize {
            random_coordinates.insert(random_coordinate(self.x_size, self.y_size));
        }

        // For testing only!!
        println!("mines are at: {:?}", random_coordinates);
         
        // 1. My approach (erroneous)
        // This approach introduced repeating coordinates for mines causing the check_mine test to fail indeterministically
        // let mine_coordinates: Vec<Coordinate> = (0..number_of_mines)
        //     .map(|_| {
        //         let random_x = rand::thread_rng().gen_range(0..self.xsize);
        //         let random_y = rand::thread_rng().gen_range(0..self.ysize);
        //         Coordinate { x: random_x, y: random_y }
        //     }) 
        //     .collect(); 

        // 2. shuffling (better for denser mines)
        // use rand::seq::SliceRandom;

        // let mut all_coordinates: Vec<Coordinate> = all_coordinates(self.xsize, self.ysize)
        // all_coordinates.shuffle(&mut rand::thread_rng()); // shuffle inplace

        // let mine_coordinates: Vec<Coordinate> = all_coordinates
        //     .into_iter()
        //     .take(number_of_mines)
        //     .collect();

        self.place_mines_at(&random_coordinates)
    }

    pub fn get_player_board(&self) -> PlayerBoard {
        let mut player_board_map = HashMap::new();

        for (coordinate, tile) in self.board_map.clone() { // .clone() is necessary 
                                                                                // self.board_map is "moved" in the for loop
            let player_tile = match tile.status {
                TileStatus::Flagged => PlayerTile::Flagged,
                TileStatus::Hidden => PlayerTile::Hidden,
                TileStatus::Revealed => match tile.has_mine {
                    true => PlayerTile::Mine,
                    false => PlayerTile::Hint(self.num_mines_nearby(&coordinate))
                }

            };

            player_board_map.insert(coordinate, player_tile);
        }

        Board {
            x_size: self.x_size,
            y_size: self.y_size,
            board_map: player_board_map,
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
        potential_coordinate.0 >= 0 && potential_coordinate.0 < self.x_size as i32 && 
        potential_coordinate.1 >= 0 && potential_coordinate.1 < self.y_size as i32
    }

    pub fn update(&self, player_action: &PlayerAction) -> RefBoard {
        let coordinate = player_action.coordinate;
        let updated_tile = self.board_map.get(&coordinate).unwrap().update(&player_action);

        let mut updated_board_map = self.board_map.clone();

        updated_board_map.insert(coordinate, updated_tile);

        RefBoard {
            x_size: self.x_size,
            y_size: self.y_size,
            board_map: updated_board_map
        }
    }
}


//////////////////////////////////////////////////////////////////
// Support Functions
//////////////////////////////////////////////////////////////////
pub fn random_coordinate(x_size: u32, y_size: u32) -> Coordinate {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    Coordinate {
        x: rng.gen_range(0..x_size),
        y: rng.gen_range(0..y_size),
    }
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

type PlayerBoard = Board<PlayerTile>;

impl PlayerBoard {
    pub fn print(&self) {
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                match self.board_map.get(&Coordinate{ x, y }).unwrap() {
                    PlayerTile::Hidden => print!("? "),
                    PlayerTile::Flagged => print!("! "),
                    PlayerTile::Hint(n) => print!("{} ", n),
                    PlayerTile::Mine => print!("* ")
                }
            }
            println!();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*; // bring all of the items belonging to the tests module’s parent into scope

    #[test]
    fn check_mines() {
        let board = Board::new(5, 5);
        let board_with_mines = board.place_mines(5);
        
        let count = board_with_mines.board_map
            .values()
            .filter(|t| t.has_mine)
            .count();

        assert_eq!(count, 5);
    }

    fn test_update() {
        let board = Board::new(2, 2);
        let mine_coordinate = HashSet::from([Coordinate{x: 0, y:0}]);
        
        let board_with_mines = board.place_mines_at(&mine_coordinate);

        let updated_board = board_with_mines.update(&PlayerAction{coordinate: Coordinate{x: 0, y: 0}, action: Action::Flag});
        let updated_tile = updated_board.board_map.get(mine_coordinate.iter().next().unwrap()).unwrap();

        assert_eq!(updated_tile.status, TileStatus::Flagged)
    }
}