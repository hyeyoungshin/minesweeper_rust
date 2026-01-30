mod game;
mod text_ui;

use crate::game::*;
use crate::game::player::*;
use crate::text_ui::*;
use std::io;

fn main() -> io::Result<()> {
    start_game();

    let player = Player::new(get_id()?, 0);

    let (h_size, v_size) = get_board_size()?;
    // TODO: validate input here too
    let game_level = get_difficulty()?;

    let mut game = Game::new(h_size, v_size, game_level);
    
    ////////// interactive game loop //////////
    while game.status == GameStatus::Continue {
        // 1. get player's coordinate
        let player_coordinate = get_coordinate(&game)?;
        println!("player coordinate: {:?}", player_coordinate);
        
        // 2. get player's action
        let player_action = get_action(&game, &player_coordinate)?;
        println!("player action: {:?}", player_action);

        // 3. update the game
        game = game.update(&PlayerAction{ player: player.clone(), coordinate: player_coordinate, action: player_action });

        // 4. print board
        game.board.print();
    }

    end_game(&game);

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
        
    //     game.ref_board.get_player_board().print();
    // }

    // println!("{:?}", game.status);
}
