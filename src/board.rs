use std::collections::HashMap;
use rand::Rng;
use crate::game::Difficulty;

const DEFAULT_SIZE: u8 = 3;

pub struct Board<Tile> { // making Tile 1. a parameter 2. a trait
                         // depends on whether Board needs to interact with Tile in its implementation or not
    xsize: u32, // horizontal size (grows to right)
    ysize: u32, // vertical size (grows down)
    board_map: HashMap<Coordinate, Tile>, // invariant: `board_map` stores precisely `xsize` * `ysize` entries
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
    has_mine: bool,
    status: TileStatus,
}

#[derive(Clone)]
enum TileStatus {
    Hidden,
    Flagged,
    Revealed
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Coordinate { x: u32, y: u32 }

type RefBoard = Board<RefTile>;

impl RefBoard {
    fn default() -> Self {
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

    pub fn plant_mines(&self, d: Difficulty) -> RefBoard {
        let size = self.xsize * self.ysize;
        let number_of_mines: f32 = size as f32 * { 
            match d {
                Difficulty::Easy => 0.12,
                Difficulty::Medium => 0.15,
                Difficulty::Hard => 0.2 
            }
        };

        let number_of_mines: u32 = number_of_mines.floor() as u32;

        let mine_coordinates: Vec<Coordinate> = (0..number_of_mines)
            .map(|_| {
                let random_x = rand::thread_rng().gen_range(0..self.xsize);
                let random_y = rand::thread_rng().gen_range(0..self.ysize);
                Coordinate { x: random_x, y: random_y }
            })
            .collect(); 

        let mut new_board_map: HashMap<Coordinate, RefTile> = HashMap::new();
        
        for coordinate in mine_coordinates {
            new_board_map.insert(coordinate, RefTile{has_mine: true, status: TileStatus::Hidden});
        }

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
            
            if self.is_valid(&potential_coordinate) { // check if it's outside board boundaries 
                                                      // if true, potential_coordinate.0 and potential_coordinate.1 is always positive
                neighbors_coordinates.push(Coordinate{x: potential_coordinate.0 as u32, y: potential_coordinate.1 as u32}); // i32 to u32 ok???
            }   
        }
        
        neighbors_coordinates.len() as i8
    }

    fn is_valid(&self, potential_coordinate: &(i32, i32)) -> bool {
        potential_coordinate.0 >= 0 && potential_coordinate.0 < self.xsize as i32 && 
        potential_coordinate.1 >= 0 && potential_coordinate.1 < self.ysize as i32
    }
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

