mod game;

use crate::game::*;
use game::board::*;
use std::io;
// use crate::game::Difficulty;
// use crate::game::GameStatus;

// qustion: the game argument is not necessary because .is_valid_coordinate can be a helper function instead. 
// but thought it might be nice to make it depend on the game being played... but now parsing is not separated from game.
// parse_action does not depend on game.
// So what does Michael think about this?
fn parse_coordinate(game: &Game, mut player_input: String) -> Result<Coordinate, Box<dyn std::error::Error>> {
    io::stdin().read_line(&mut player_input)?;

    let chars: Vec<&str> = player_input.trim().split(',').collect();

    if chars.len() != 2 {
        return Err("Expected x,y".into());
    }

    let x: u32 = chars[0].parse()?;
    let y: u32 = chars[1].parse()?;

    match game.is_valid_coordinate(&Coordinate{x: x, y: y}) {
        true => Ok(Coordinate{x: x, y: y}),
        false => Err("Coordinates out of bounds".into()) // .into() performs a type conversion. 
        // It converts a value from one type into another type that the compiler can infer from context.
        // Without it, Err(Box::<dyn std::error::Error>::from("Coordinates out of bounds"))
    }
}

fn parse_action(mut player_input: String) -> Result<Action, Box<dyn std::error::Error>> {
    io::stdin().read_line(&mut player_input)?;

    match player_input.as_str() {
        "Reveal" => Ok(Action::Reveal),
        "Flag" => Ok(Action::Flag),
        "Unflag" => Ok(Action::Unflag),
        _ => Err("Wrong action command".into())
    }
}

fn main() -> io::Result<()> {

    let mut game = new_game(5,5, Difficulty::Medium);
    
    ////////// interactive game loop //////////
    while game.status == GameStatus::Continue {
        println!("Enter a coordinate separated by a comma:");
        let mut player_input = String::new();

        let parsed = parse_coordinate(&game, player_input);

        match parsed {
            Ok(coordinate) => {
                println!("Enter a move: flag, unflag, reveal?");
                let mut player_input = String::new();
                let parsed = parse_action(player_input);

                match parsed {
                    Ok(action) => {
                        game = game.make_move(&PlayerAction { coordinate: coordinate, action: action })
                    },
                    Err(msg) => {
                        println!("{:?}", msg);
                        continue; // TODO: go back to getting player input for action
                    }
                }
            },
            Err(msg) => {
                println!("{:?}", msg);
                continue;
            }
        }
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
        
    //     game.ref_board.print_mines();
    //     println!();
        
    //     game.ref_board.get_playerboard().print();
    // }

    // println!("{:?}", game.status);


    ////////////// The Game Loop //////////////
    // Used for testing
    // 
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
