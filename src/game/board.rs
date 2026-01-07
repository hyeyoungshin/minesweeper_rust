use std::collections::HashMap;
use std::collections::HashSet;
// use rand::Rng;
use crate::game::Difficulty;
use crate::parse::ValidationError;


// Board's vertical and horizontal max size 
// It is set so that we can convert u32 to i32 safely during coordinate validation
const MAX_SIZE: u32 = i32::MAX as u32; // 2147483647 

pub struct Board<Tile> { // making Tile 1. a parameter 2. a trait
                        // depends on whether Board needs to interact with Tile in its implementation or not
    pub x_size: u32, // horizontal size (grows to right)
    pub y_size: u32, // vertical size (grows down)
    pub board_map: HashMap<Coordinate, Tile>, // invariant: `board_map` stores precisely `xsize` * `ysize` entries
                                                // board_map.get(&Coordinate{ x, y }) should never return None
                                                // *if* the coordinate is valid    
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

#[derive(Clone, Copy, PartialEq)]
pub enum TileStatus {
    Hidden,
    Flagged,
    Revealed
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Coordinate { pub x: u32, pub y: u32 }

pub type RefBoard = Board<RefTile>;

impl RefBoard {
    pub fn new(xsize: u32, ysize: u32, num_mines: u32) -> Self {
        let mut board_map = HashMap::new();

        // initialize all tiles
        for x in 0..xsize {
            for y in 0..ysize {
                board_map.insert(Coordinate{ x, y }, RefTile { has_mine: false, status: TileStatus::Hidden });
            }
        }
        
        // place mines
        Board {
            x_size: xsize,
            y_size: ysize,
            board_map,
        }.place_mines(num_mines)
    }

    fn place_mines(&self, num_mines: u32) -> RefBoard {
        let size = self.x_size * self.y_size;
        // let number_of_mines: f32 = size as f32 * { 
        //     match difficulty {
        //         Difficulty::Easy => 0.12,
        //         Difficulty::Medium => 0.15,
        //         Difficulty::Hard => 0.2 
        //     }
        // };

        // let number_of_mines = number_of_mines.floor() as usize;

        // 3. HashSet (Makes most sense and idiomatic)
        let mut mine_coordinates: HashSet<Coordinate> = HashSet::new();

        while mine_coordinates.len() < num_mines as usize {
            mine_coordinates.insert(random_coordinate(self.x_size, self.y_size));
        }

        // For testing only!!
        println!("mines are at: {:?}", mine_coordinates);
         
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

        let mut new_board_map: HashMap<Coordinate, RefTile> = HashMap::new();
        
        for coordinate in mine_coordinates.clone() {
            new_board_map.insert(coordinate, RefTile{has_mine: true, status: TileStatus::Hidden});
        }

        for x in 0..self.x_size {
            for y in 0..self.y_size {
                new_board_map.entry(Coordinate{x, y}).or_insert(RefTile{has_mine: false, status: TileStatus::Hidden});
            }
        }

        Board{
            x_size: self.x_size,
            y_size: self.y_size,
            board_map: new_board_map,
        }
    }

    pub fn get_playerboard(&self) -> PlayerBoard {
        let mut player_board_map = HashMap::new();

        for (coordinate, tile) in self.board_map.clone() { // .clone() is necessary 
                                                                                // self.board_map is "moved" in the for loop
            let playertile = match tile.status {
                TileStatus::Flagged => PlayerTile::Flagged,
                TileStatus::Hidden => PlayerTile::Hidden,
                TileStatus::Revealed => match tile.has_mine {
                    true => PlayerTile::Mine,
                    false => PlayerTile::Hint(self.num_mines_nearby(&coordinate))
                }

            };

            player_board_map.insert(coordinate, playertile);
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

    pub fn mine_coordinates(&self) -> Vec<Coordinate> {
        self.board_map.clone()
            .into_iter()
            .filter(|(_, tile)| tile.has_mine)
            .map(|(coordinate,_)| coordinate)
            .collect()
    }
}


//////////////////////////////////////////////////////////////////
// Helpers
//////////////////////////////////////////////////////////////////
pub fn random_coordinate(x_size: u32, y_size: u32) -> Coordinate {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    Coordinate {
        x: rng.gen_range(0..x_size),
        y: rng.gen_range(0..y_size),
    }
}

// Generates all valid coordinates of the tiles of a board of xsize * ysize
pub fn all_coordinates(xsize: u32, ysize: u32) -> Vec<Coordinate> {
    return (0..xsize)
        .flat_map(|x| (0..ysize).map(move |y| Coordinate { x, y }))
        .collect();
}

pub fn validate_board_size(hsize: i32, vsize: i32) -> Result<(u32, u32), ValidationError> {
    if hsize > MAX_SIZE as i32 && vsize > MAX_SIZE as i32 {
        Err(ValidationError::MaxExceeded)
    } else if hsize < 0 && vsize < 0 {
        Err(ValidationError::NegativeSize)
    } else {
        Ok((hsize as u32, vsize as u32))
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
        let board = Board::new(5, 5, 5);
        // let mut count = 0;

        // for (_, v) in &board_with_mines.board_map { // without & for loop takes ownership of board_with_mines.board_map
        //     if v.has_mine  {count += 1;}
        // }
        // // without &, board_with_mines.board_map is now gone!


        // better because
        // 1. does not use a mutable counter
        // 2. more concise and readable 
        // 3. funtional style that Rustaceans prefer
        let count = board.board_map
            .values()
            .filter(|t| t.has_mine)
            .count();

        assert_eq!(count, 3); // 5 * 5 board easy should have 3 mines
    }
}