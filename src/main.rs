mod game;
mod board;

use board::Board;

fn main() {
    let ref_board = Board::new(4, 4);
    let player_board = ref_board.get_playerboard();
    player_board.print(); // refboard should not be printable
                          // maybe for the debugging purpose, we can print it
                          // playerboard should be inferred from refboard
}
