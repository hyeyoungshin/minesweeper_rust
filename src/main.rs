use std::collections::HashMap;

const DEFAULT_SIZE: u8 = 3;

struct Board {
    xsize: u32, // horizontal size (grows to right)
    ysize: u32, // vertical size (grows down)
    board_map: HashMap<Coordinate, PlayerTile>, // invariant: `board_map` stores precisely `xsize` * `ysize` entries
                                                // board_map.get(&Coordinate{ x, y }) should never return None
                                                // so if it does
                                                // .unwrap() -> crash 
                                                // .expect("err msg") -> print err msg
}

#[derive(Debug, Clone, PartialEq)]
enum PlayerTile {
    Hidden,
    Flagged,
    Revealed(RevealedTile) // improved design from `Revealed(i8)`
}

#[derive(Debug, Clone, PartialEq)]
enum RevealedTile {
    Hint(i8), // i8 suffices since # of mines in neighboring tiles cannot exceed 8
    Mine
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Coordinate { x: u32, y: u32 }

impl Board {
    fn default() -> Self {
        Board::new(DEFAULT_SIZE as u32, DEFAULT_SIZE as u32) // u8 -> u32 upcase (safe)
    }

    fn new(xsize: u32, ysize: u32) -> Self {
        let mut board_map = HashMap::new(); // mutate locally
                                                                             // while keeping the functional style globally
                                                                             // by generating a new `board_map` whenever a player makes a move 

        for x in 0..xsize {
            for y in 0..ysize {
                board_map.insert(Coordinate{ x, y }, PlayerTile::Hidden);
            }
        }

        Board {
            xsize,
            ysize,
            board_map,
        }
    }

    fn print(&self) {
        for y in 0..self.ysize {
            for x in 0..self.xsize {
                match self.board_map.get(&Coordinate{ x, y }).unwrap() {
                    PlayerTile::Hidden => print!("? "),
                    PlayerTile::Flagged => print!("! "),
                    PlayerTile::Revealed(value) => match value {
                        RevealedTile::Hint(n) => print!("{}", n),
                        RevealedTile::Mine => print!("*")
                    }
                }
            }
            println!();  // New line after each row
        }
    }
}

fn main() {
    let db = Board::default();
    db.print();
    println!();
    let b2 = Board::new(4, 4);
    b2.print();
}
