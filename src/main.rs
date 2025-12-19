mod game;

use crate::game::*;
use game::board::*;
use std::io;
// use crate::game::Difficulty;
// use crate::game::GameStatus;



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

fn main() -> io::Result<()> {
    let mut game = new_game(5,5, Difficulty::Medium);


    while game.status == GameStatus::Continue {
        println!("Enter a coordinate separated by a comma:");

        let mut player_input = String::new();
        

        let parsed = parse_coordinate(&game, player_input);
        
        match parsed {
            Ok(coordinate) => {
                println!("Enter a move: flag, unflag, reveal?");
                //TODO: implement parse_player_action
                //let game = game.make_move(player_action)
            },
            Err(msg) => {
                println!("{:?}", msg);
                continue;
            }
        }
    }

    Ok(())

    /////// automatic game play /////////
    // plays until game status becomes either over or error
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


    ////////////// The Game Loop ////////////
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
