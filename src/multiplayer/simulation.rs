use std::io;
use rand::Rng;

use crate::core::game::{Game};
use crate::core::game::*;
use crate::single_player::text_ui::*;

pub fn simulate_multiplayer() -> io::Result<Game> {
    let mut game = start_game();
    
    let mut rng = rand::thread_rng();
    
    while game.status == GameStatus::Continue {
        let turn_id = rng.gen_range(1..game.players.len()+1) as u32;
        let current_player = game.get_player(&turn_id);

        println!("{}'s turn", current_player.name);

        let coordinate = get_coordinate(&game, current_player)?;
        let action = get_action(&game, current_player, coordinate)?;

        println!("{}'s move: {:?} {:?}", current_player.name, action.action, coordinate);

        game = game.update(&action);
        game.board.print();
        print_scores(&game);
    }

    announce_winners(&game);

    Ok(game)
}