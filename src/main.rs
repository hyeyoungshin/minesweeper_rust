mod game;

use crate::game::*;
use game::board::*;
use std::io;
// use crate::game::Difficulty;
// use crate::game::GameStatus;


fn main() {
    let mut game = new_game(5,5, Difficulty::Medium);


    // while game.status == GameStatus::Continue {
    //     println!("Enter a coordinate separated by a comma:");

    //     let mut player_coordinate = String::new();
    //     io::stdin().read_line(&mut player_coordinate)?;

    //     let chars: Vec<&str> = player_coordinate.trim().split(',').collect();
        
    //     // println!("{:?}", chars)
    //     if is_valid_input(chars) {
    //         // continue
    //     } else {
    //         //
    //     }
    // }

    // Ok(())

    /////// automatic game play /////////
    // plays until game status becomes either over or error
    while game.status == GameStatus::Continue {
            let player_coordinate = random_coordinate(game.ref_board.x_size, game.ref_board.y_size);
        println!("player coordinate: {:?}", player_coordinate);
        
        // println!("{}", game.ref_board.num_mines_nearby(&player_coordinate));
        
        let player_action = random_action();
        println!("player action: {:?}", player_action);
        
        let action = &PlayerAction {
            coordinate: player_coordinate,
            action: player_action
        };
        
        game = game.make_move(action);
        
        game.ref_board.print_mines();
        println!();
        
        game.ref_board.get_playerboard().print();
    }

    println!("{:?}", game.status);


    ////////////// The Game Loop ////////////
    // while game.status == GameStatus::Continue {
    //     println!("game status:{:?}", game.status);

    //     let player_coordinate = random_coordinate(game.ref_board.x_size, game.ref_board.y_size);
    //     println!("coordinate: {:?}", player_coordinate);
        
    //     let player_action = random_action();
    //     println!("action: {:?}", player_action);
        
    //     let action = &PlayerAction {
    //         coordinate: player_coordinate,
    //         action: Action::Flag,
    //     };
        
    //     game = game.make_move(action);
    //     if game.status == GameStatus::Error {
    //         println!("Invalid move! Make another move.");
    //         game.status = GameStatus::Continue
    //     }
    // }
    // println!("game status:{:?}", game.status);

}
