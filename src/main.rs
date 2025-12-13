mod game;
mod board;

use board::Board;
use game::Difficulty;

fn main() {
    let ref_board = Board::new(4, 4);
    let player_board = ref_board.get_playerboard();
    player_board.print(); // refboard should not be printable
                          // maybe for the debugging purpose, we can print it
                          // playerboard should be inferred from refboard
    println!();

    let ref_board_with_mines = ref_board.plant_mines(&game::Difficulty::Hard);
    let new_player_board = ref_board_with_mines.get_playerboard();
    new_player_board.print();
}


#[cfg(test)]
mod tests {
    use super::*;

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
            .filter(|v| v.has_mine)
            .count();

        assert_eq!(count, 3);
    }
}