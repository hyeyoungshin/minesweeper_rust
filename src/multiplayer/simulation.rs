use std::io;

use crate::core::player::Player;
use crate::core::game::{Game, Difficulty};
use crate::core::game::*;
use crate::single_player::text_ui::*;

pub fn simulate_turn_based() -> io::Result<()> {
    start_game();
    let mut game = Game::new(10, 10, Difficulty::Medium)
        .add_player(Player::new("hyeyoung".to_string()))
        .add_player(Player::new("charlie".to_string()))
        .add_player(Player::new("william".to_string()));

    while game.status == GameStatus::Continue {
        for i in 0..game.turn_order.len() {
            let player_id = game.turn_order[i];
            let player = game.get_player(&player_id);
            
            let coordinate = get_coordinate(&game, player)?;
            // println!("{}'s coordinate: {:?}", player.name, coordinate);
            
            let action = get_action(&game, player, coordinate)?;
            println!("{}'s move: {:?} {:?}", player.name, action.action, coordinate);

            game = game.update(&action); // TODO: points added here
                                         // if a player reveals a mine, -10
            game.board.print();
        }
    }
    end_game(&game);
    Ok(())
}