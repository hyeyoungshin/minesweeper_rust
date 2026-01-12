mod game;
mod text_ui;

use crate::game::*;
use crate::text_ui::*;
use std::io;

fn main() -> io::Result<()> {
    println!("Let's play minesweeper game!");

    let (hsize, vsize) = get_board_size()?;
    // TODO: validate input here too
    let num_mines = get_num_mines()?;

    let mut game = new_game(hsize, vsize, num_mines);
    
    ////////// interactive game loop //////////
    while game.status == GameStatus::Continue {
        // 1. get player's coordinate
        let player_coordinate = get_coordinate(&game)?;
        println!("player coordinate: {:?}", player_coordinate);
        
        // 2. get player's action
        let player_action = get_action(&game, &player_coordinate)?;
        println!("player action: {:?}", player_action);

        // 3. update the game
        game = game.update(&PlayerAction{ coordinate: player_coordinate, action: player_action });

        // 4. print player's board
        let player_board = game.ref_board.get_player_board();
        player_board.print();
    }

    // TODO: write a function for this
    match game.status {
        GameStatus::Win => println!("You won!"),
        GameStatus::Over => println!("You lost..."),
        _ => {panic!("should not be here");}

    }

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



// TODO: Fix this (Jan 5)
// Error scenario #1:

// player action: Reveal
// 1 ? 
// 1 1 
// Enter a coordinate: x,y
// 0,1
// tile at Coordinate { x: 0, y: 1 } already revealed
// 1,0   
// player coordinate: Coordinate { x: 1, y: 0 }
// Enter an action: Flag, Unflag, or Reveal
// Flag
// player action: Flag
// 1 ! 
// 1 1 
// You won!


// Error scenario #2:

// player coordinate: Coordinate { x: 0, y: 1 }
// Enter an action: Flag, Unflag, or Reveal
// Reveal
// player action: Reveal
// 1 1 
// 1 ! 
// Enter a coordinate: x,y
// 1,1
// player coordinate: Coordinate { x: 1, y: 1 }
// Enter an action: Flag, Unflag, or Reveal
// Unflag
// player action: Unflag
// 1 1 
// 1 ? 
// You won!