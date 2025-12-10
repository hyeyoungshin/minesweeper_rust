use std::collections::HashMap;
use rand::Rng;
use crate::game::Difficulty;

const DEFAULT_SIZE: u8 = 3;

pub struct Board {
    xsize: u32, // horizontal size (grows to right)
    ysize: u32, // vertical size (grows down)
    board_map: HashMap<Coordinate, TileStatus>, // invariant: `board_map` stores precisely `xsize` * `ysize` entries
                                                // board_map.get(&Coordinate{ x, y }) should never return None
                                                // so if it does
                                                // .unwrap() -> crash 
                                                // .expect("err msg") -> print err msg
}

#[derive(Debug, Clone, PartialEq)]
enum TileStatus {
    Hidden,
    Flagged,
    Revealed(TileValue) // improved design from `Revealed(i8)`
}

#[derive(Debug, Clone, PartialEq)]
enum TileValue {
    Hint(i8), // i8 suffices since # of mines in neighboring tiles cannot exceed 8
    Mine
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Coordinate { x: u32, y: u32 }

impl Board {
    fn default() -> Self {
        Board::new(DEFAULT_SIZE as u32, DEFAULT_SIZE as u32) // u8 -> u32 upcase (safe)
    }

    pub fn new(xsize: u32, ysize: u32) -> Self {
        let mut board_map = HashMap::new(); // mutate locally
                                                                             // while keeping the functional style globally
                                                                             // by generating a new `board_map` whenever a player makes a move 

        for x in 0..xsize {
            for y in 0..ysize {
                board_map.insert(Coordinate{ x, y }, TileStatus::Hidden);
            }
        }

        Board {
            xsize,
            ysize,
            board_map,
        }
    }

    pub fn print(&self) {
        for y in 0..self.ysize {
            for x in 0..self.xsize {
                match self.board_map.get(&Coordinate{ x, y }).unwrap() {
                    TileStatus::Hidden => print!("? "),
                    TileStatus::Flagged => print!("! "),
                    TileStatus::Revealed(value) => match value {
                        TileValue::Hint(n) => print!("{}", n),
                        TileValue::Mine => print!("* ")
                    }
                }
            }
            println!();
        }
    }

    pub fn plant_mines(&self, d: Difficulty) -> Board {
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

        let mut new_board_map: HashMap<Coordinate, TileStatus> = HashMap::new();
        
        for coordinate in mine_coordinates {
            new_board_map.insert(coordinate, TileStatus::Revealed(TileValue::Mine));
        }

        for x in 0..self.xsize {
            for y in 0..self.ysize {
                new_board_map.entry(Coordinate{x, y}).or_insert(TileStatus::Hidden);
            }
        }

        Board{
            xsize: self.xsize,
            ysize: self.ysize,
            board_map: new_board_map
        }
    }
}
