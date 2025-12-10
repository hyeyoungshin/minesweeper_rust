mod game;
mod board;

use board::Board;

fn main() {
    let b2 = Board::new(4, 4);
    b2.print();

    println!();
    let new_board = b2.plant_mines(game::Difficulty::Medium);
    new_board.print();
    
}
