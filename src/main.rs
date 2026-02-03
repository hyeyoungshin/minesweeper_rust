mod game;
mod text_ui;

use crate::game::*;
use crate::text_ui::*;
use crate::game::player::*;
use std::io;


fn main() -> io::Result<()> {
    start_game();

    let player = Player::new(get_name()?);

    let (h_size, v_size) = get_board_size()?;
    // TODO: validate input here too
    let game_level = get_difficulty()?;

    let mut game = Game::new(h_size, v_size, game_level);
    
    ////////// interactive game loop //////////
    while game.status == GameStatus::Continue {
        // 1. get player's coordinate
        let player_coordinate = get_coordinate(&game, &player)?;
        println!("player coordinate: {:?}", player_coordinate);
        
        // 2. get player's action
        let player_action = get_action(&game, &player, player_coordinate)?;
        println!("player action: {:?}", player_action);

        // 3. update the game
        game = game.update(&player_action);

        // 4. print board
        game.board.print();
    }

    end_game(&game);

    Ok(())
}
