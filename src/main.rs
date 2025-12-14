mod game;
mod board;

fn main() {
    let ref_board = board::RefBoard::new(4, 4);
    let player_board = ref_board.get_playerboard();
    player_board.print(); // refboard should not be printable
                          // maybe for the debugging purpose, we can print it
                          // playerboard should be inferred from refboard
    println!();

    let ref_board_with_mines = ref_board.plant_mines(&game::Difficulty::Hard);
    let new_player_board = ref_board_with_mines.get_playerboard();
    new_player_board.print();
}
