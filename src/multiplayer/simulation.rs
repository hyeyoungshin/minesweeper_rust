use std::io;

use crate::core::player::Player;
use crate::core::game::{Game, Difficulty};
use crate::core::game::*;
use crate::single_player::text_ui::*;

pub fn simulate_turn_based() -> io::Result<()> {
    start_game();
    let mut game = Game::new(3, 3, Difficulty::Medium)
        .add_player(Player::new("hyeyoung".to_string()))
        .add_player(Player::new("charlie".to_string()))
        .add_player(Player::new("william".to_string()));
    
    let players = game.players.clone();
    
    while game.status == GameStatus::Continue {
        for id in players.keys() {
            let current_player = game.get_player(&id);

            let coordinate = get_coordinate(&game, current_player)?;
            let action = get_action(&game, current_player, coordinate)?;

            println!("{}'s move: {:?} {:?}", current_player.name, action.action, coordinate);

            game = game.update(&action);
            game.board.print();
            if game.status == GameStatus::Over {
                break;
            }
        }
    }

    end_game(&game);
    Ok(())
}