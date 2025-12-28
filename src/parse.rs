use crate::game::*;
use crate::game::board::*;

use std::io;

pub fn parse_coordinate(player_input: String) -> Result<Coordinate, Box<dyn std::error::Error>> {
    let chars: Vec<&str> = player_input.trim().split(',').collect();

    match chars.len() {
        2 => {
            let x: u32 = chars[0].parse()?;
            let y: u32 = chars[1].parse()?;

            Ok(Coordinate{x: x, y: y})
        },
        _ => Err("Expected x,y format".into())
    }
}

pub fn parse_action(player_input: String) -> Result<Action, Box<dyn std::error::Error>> {
    match player_input.trim() {
        "Reveal" => Ok(Action::Reveal),
        "Flag" => Ok(Action::Flag),
        "Unflag" => Ok(Action::Unflag),
        _ => Err("Wrong action command".into())
    }
}

pub fn get_coordinate(game: &Game) -> io::Result<Coordinate> {
    println!("Enter a coordinate: x,y");
    // The loop continues until one branch hits return Ok(valid_coord)
    loop {
        let mut player_input = String::new();
        io::stdin().read_line(&mut player_input)?;
         
        // Approach 1:
        // This chains parse and validate together
        // match parse_coordinate(player_input)
        //     .ok()
        //     .and_then(|coord| game.validate_coordinate(&coord))
        // {
        //     Some(coordinate) => return Ok(coordinate),
        //     None => println!("Invalid coordinate"),
        // }
        
        // Approach 2: Better design because more explicit 
        // This handles both error sources separately, and preserves error messages.
        
        match parse_coordinate(player_input) {
            Ok(coord) => {
                match game.validate_coordinate(&coord) {
                    Some(coord) => return Ok(coord), // all match arms return ()
                    None => println!("tile already revealed at {:?} or Coordinate out of bounds", coord),
                }
            }, 
            Err(msg) => println!("Parse error: {}", msg),
        }
    }
}


pub fn get_action(game: &Game, coordinate: &Coordinate) -> io::Result<Action> {
    println!("Enter an action: Flag, Unflag, or Reveal");

    loop {
        let mut player_input = String::new();
        io::stdin().read_line(&mut player_input)?; // catches erros from OS
        
        match parse_action(player_input) {
            Ok(action) => match game.validate_action(action, coordinate) {
                Some(action) => return Ok(action),
                None => println!("invalid action")
            }
            Err(msg) => println!("Parse error: {}", msg),
        }
    }
}
