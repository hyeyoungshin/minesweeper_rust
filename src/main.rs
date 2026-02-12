use std::io;

// use minesweeper_rust::core::player::Player;
// use minesweeper_rust::core::game::{Game, GameStatus};
use minesweeper_rust::multiplayer::simulation::*;
// use minesweeper_rust::single_player::text_ui::*;

fn main() -> io::Result<()> {
    let result = simulate_turn_based();

    match result {
        Ok(_) => println!("it worked!"),
        Err(_) => println!("it didn't work...")
    }

    // start_game();

    // let single_player = Player::new(get_name()?);

    // let (h_size, v_size) = get_board_size()?;
    // // TODO: validate input here too
    // let game_level = get_difficulty()?;

    // let mut game = Game::new(h_size, v_size, game_level);
    
    // game = game.add_player(single_player);
    
    // ////////// interactive game loop //////////
    // while game.status == GameStatus::Continue {
    //     // 1. get player's coordinate
    //     let player_coordinate = get_coordinate(&game, &game.get_player(&1))?;
    //     println!("player coordinate: {:?}", player_coordinate);
        
    //     // 2. get player's action
    //     let player_action = get_action(&game, &game.get_player(&1), player_coordinate)?;
    //     println!("player action: {:?}", player_action);

    //     // 3. update the game
    //     game = game.update(&player_action);

    //     // 4. print board
    //     game.board.print();
    // }

    // end_game(&game);

    Ok(())
}
