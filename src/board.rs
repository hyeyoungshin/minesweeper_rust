use std::collections::HashMap;
use std::collections::HashSet;
use rand::Rng;
use crate::game::Difficulty;


const DEFAULT_SIZE: u8 = 3;

pub struct Board<Tile> { // making Tile 1. a parameter 2. a trait
                        // depends on whether Board needs to interact with Tile in its implementation or not
    pub xsize: u32, // horizontal size (grows to right)
    pub ysize: u32, // vertical size (grows down)
    pub board_map: HashMap<Coordinate, Tile>, // invariant: `board_map` stores precisely `xsize` * `ysize` entries
                                                // board_map.get(&Coordinate{ x, y }) should never return None
                                                // so if it does
                                                // .unwrap() -> crash 
                                                // .expect("err msg") -> print err msg
}

// Tile presentation for players
#[derive(Debug, Clone, PartialEq)]
pub enum PlayerTile {
    Hidden,
    Flagged,
    Hint(i8), // i8 suffices since # of mines in neighboring tiles cannot exceed 8
    Mine
}

#[derive(Clone)]
pub struct RefTile {
    pub has_mine: bool,
    pub status: TileStatus,
}

#[derive(Clone)]
pub enum TileStatus {
    Hidden,
    Flagged,
    Revealed
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Coordinate { pub x: u32, pub y: u32 }

pub type RefBoard = Board<RefTile>;

impl RefBoard {
    pub fn default() -> Self {
        Board::new(DEFAULT_SIZE as u32, DEFAULT_SIZE as u32) // u8 -> u32 upcase (safe)
    }

    pub fn new(xsize: u32, ysize: u32) -> Self {
        let mut board_map = HashMap::new(); // mutate locally
                                                                        // while keeping the functional style globally
                                                                        // by generating a new `board_map` whenever a player makes a move 

        for x in 0..xsize {
            for y in 0..ysize {
                board_map.insert(Coordinate{ x, y }, RefTile {has_mine: false, status: TileStatus::Hidden});
            }
        }

        Board {
            xsize,
            ysize,
            board_map,
        }
    }

    pub fn plant_mines(&self, d: &Difficulty) -> RefBoard {
        let size = self.xsize * self.ysize;
        let number_of_mines: f32 = size as f32 * { 
            match d {
                Difficulty::Easy => 0.12,
                Difficulty::Medium => 0.15,
                Difficulty::Hard => 0.2 
            }
        };

        let number_of_mines = number_of_mines.floor() as usize;

        // 3. HashSet (Makes most sense and idiomatic)
        let mut mine_coordinates: HashSet<Coordinate> = HashSet::new();
        let mut rng = rand::thread_rng();

        while mine_coordinates.len() < number_of_mines as usize {
            let coordinate = Coordinate {
                x: rng.gen_range(0..self.xsize),
                y: rng.gen_range(0..self.xsize),
            };
            mine_coordinates.insert(coordinate);

        }
        
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
        
        for coordinate in mine_coordinates {
            new_board_map.insert(coordinate, RefTile{has_mine: true, status: TileStatus::Hidden});
            println!("mine coordinate is:{:?}", (coordinate.x, coordinate.y));
        }

        println!("keys in board_map: {:?}", new_board_map.keys()); // 2 sometimes... why?

        for x in 0..self.xsize {
            for y in 0..self.ysize {
                new_board_map.entry(Coordinate{x, y}).or_insert(RefTile{has_mine: false, status: TileStatus::Hidden});
            }
        }

        Board{
            xsize: self.xsize,
            ysize: self.ysize,
            board_map: new_board_map
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
            xsize: self.xsize,
            ysize: self.ysize,
            board_map: player_board_map
        }
    }

    pub fn num_mines_nearby(&self, coordinate: &Coordinate) -> i8 {
        let relative_coordinates:[(i32, i32); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

        let mut neighbors_coordinates: Vec<Coordinate> = Vec::new();
        
        for r_c in relative_coordinates {
            let potential_coordinate = (coordinate.x as i32 + r_c.0 , coordinate.y as i32 + r_c.1 ); // u32 as i32 is ok
            
            if is_valid(self.xsize, self.ysize, &potential_coordinate) {
                neighbors_coordinates.push(Coordinate{x: potential_coordinate.0 as u32, y: potential_coordinate.1 as u32});
            }   
        }
        
        neighbors_coordinates.len() as i8
    }

    
}

fn is_valid(xsize: u32, ysize: u32, potential_coordinate: &(i32, i32)) -> bool {
    potential_coordinate.0 >= 0 && potential_coordinate.0 < xsize as i32 && 
    potential_coordinate.1 >= 0 && potential_coordinate.1 < ysize as i32
}

fn all_coordinates(xsize: u32, ysize: u32) -> Vec<Coordinate> {
    return (0..xsize)
        .flat_map(|x| (0..ysize).map(move |y| Coordinate { x, y }))
        .collect();
}

type PlayerBoard = Board<PlayerTile>;

impl PlayerBoard {
    pub fn print(&self) {
        for y in 0..self.ysize {
            for x in 0..self.xsize {
                match self.board_map.get(&Coordinate{ x, y }).unwrap() {
                    PlayerTile::Hidden => print!("? "),
                    PlayerTile::Flagged => print!("! "),
                    PlayerTile::Hint(n) => print!("{}", n),
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
        let board_with_mines = board.plant_mines(&Difficulty::Easy); // in this example, does passing a reference to plant_mines 
                                                                                     // make sense?

        // let mut count = 0;

        // for (_, v) in &board_with_mines.board_map { // without & for loop takes ownership of board_with_mines.board_map
        //     if v.has_mine  {count += 1;}
        // }
        // // without &, board_with_mines.board_map is now gone!


        // better because
        // 1. does not use a mutable counter
        // 2. more concise and readable 
        // 3. funtional style that Rustaceans prefer
        let count = board_with_mines.board_map
            .values()
            .filter(|t| t.has_mine)
            .count();

        assert_eq!(count, 3); // 5 * 5 board easy should have 3 mines
    }
}