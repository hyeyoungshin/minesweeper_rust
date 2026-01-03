mod game;
mod parse;

use crate::game::*;
use crate::parse::*;
use std::io;

fn main() -> io::Result<()> {
    println!("Let's play minesweeper game!");

    let (hsize, vsize) = get_board_size()?;

    let mut game = new_game(hsize, vsize, Difficulty::Medium);
    
    ////////// interactive game loop //////////
    while game.status == GameStatus::Continue {
        // 1. get player's coordinate
        let player_coordinate = get_coordinate(&game)?;
        println!("player coordinate: {:?}", player_coordinate);
        
        // 2. get player's action
        let player_action = get_action(&game, &player_coordinate)?;
        println!("player action: {:?}", player_action);

        // 3. update the game
        game = game.make_move(&PlayerAction{ coordinate: player_coordinate, action: player_action });

        // 4. print player's board
        let player_board = game.ref_board.get_playerboard();
        player_board.print();
    }
    println!("Game Over!");

    Ok(())

    //////////////// automatic game play ///////////////////
    // Player's coordinate and action are randomly selected
    // The game ends when status is 
    // 1. Over --- revealed a mine 
    // 2. Error --- made an invalid move
    //
    // while game.status == GameStatus::Continue {
    //         let player_coordinate = random_coordinate(game.ref_board.x_size, game.ref_board.y_size);
    //     println!("player coordinate: {:?}", player_coordinate);
        
    //     // println!("{}", game.ref_board.num_mines_nearby(&player_coordinate));
        
    //     let player_action = random_action();
    //     println!("player action: {:?}", player_action);
        
    //     let action = &PlayerAction {
    //         coordinate: player_coordinate,
    //         action: player_action
    //     };
        
    //     game = game.make_move(action);
    //     println!();
        
    //     game.ref_board.get_playerboard().print();
    // }

    // println!("{:?}", game.status);
}
