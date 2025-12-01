use std::collections::HashMap;

struct Board {
    xsize: u32, // horizontal size (grows to right)
    ysize: u32, // vertical size (grows down)
    board_map: HashMap<(u32, u32), Tile>,
}

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Hidden,
    Flagged,
    Revealed (i32), // -1 if mine, values >= 0 indicated number of mines in vincinity
}

impl Default for Board {
    fn default() -> Self {
        let mut board_map = HashMap::new();

        for x in 0..3 {
            for y in 0..3 {
                board_map.insert((x, y), Tile::Hidden);
            }
        }

        Board {
            xsize: 3,
            ysize: 3,
            board_map,
        }
    }
}

impl Board {
    fn new(xsize: u32, ysize: u32) -> Self {
        let mut board_map = HashMap::new();

        for x in 0..xsize {
            for y in 0..ysize {
                board_map.insert((x, y), Tile::Hidden);
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
                match self.board_map.get(&(x, y)) {
                    Some(Tile::Hidden) => print!("? "),
                    Some(Tile::Flagged) => print!("! "),
                    Some(Tile::Revealed(value)) => print!("{} ", value),
                    None => print!(". "),  // Empty cell
                }
            }
            println!();  // New line after each row
        }
    }
}

fn main() {
    let b = Board::default();
    b.print();
}
