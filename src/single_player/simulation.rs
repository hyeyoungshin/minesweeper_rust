use crate::core::game::{Game, GameStatus};
use crate::single_player::text_ui::*;

use std::io;

pub fn simulate_single_player() -> io::Result<Game> {
    // println!("Let's play minesweeper game!");

    // let single_player = Player::new(get_name());

    // let (h_size, v_size) = get_board_size()?;
    // // TODO: validate input here too
    // let game_level = get_difficulty()?;

    // let mut game = Game::new(h_size, v_size, game_level);
    
    // game = game.add_player(single_player);

    // game.board.print();

    let mut game = start_game();
    
    ////////// interactive game loop //////////
    while game.status == GameStatus::Continue {
        // 1. get player's coordinate
        let player_coordinate = get_coordinate(&game, &game.get_player(&1))?;
        println!("player coordinate: {:?}", player_coordinate);
        
        // 2. get player's action
        let player_action = get_action(&game, &game.get_player(&1), player_coordinate)?;
        println!("player action: {:?}", player_action);

        // 3. update the game
        game = game.update(&player_action);

        // 4. print board
        game.board.print();
        print_scores(&game);
    }

    announce_winners(&game);

    Ok(game)
}