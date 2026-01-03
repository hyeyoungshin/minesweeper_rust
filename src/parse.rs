use crate::game::*;
use crate::game::board::*;

use std::io;

#[derive(Debug)]
pub enum ParseError {
    BadFormat,
    NotNumber(std::num::ParseIntError),
}

pub enum InvalidValue {
    OutOfBounds,
    Unavailable,
    MaxExceeded
}

pub trait FromPair {
    fn from_pair(x: u32, y: u32) -> Self;
}

impl FromPair for Coordinate {
    fn from_pair(x: u32, y: u32) -> Self {
        Coordinate { x, y }
    }
}

impl FromPair for (u32, u32) {
    fn from_pair(x: u32, y: u32) -> Self {
        (x, y)
    }
}

pub fn parse_input<T: FromPair> (player_input: String) -> Result<T, ParseError> {
    let chars: Vec<&str> = player_input.trim().split(',').collect();
    // TODO: -1,n triggers not number parse error
    match chars.len() {
        2 => {
            let x = chars[0].parse::<u32>()
                .map_err(ParseError::NotNumber)?;
            let y = chars[1].parse::<u32>()
                .map_err(ParseError::NotNumber)?;
            Ok(T::from_pair(x, y))
        },
        _ => Err(ParseError::BadFormat)
    }
}

// pub fn parse_coordinate(player_input: String) -> Result<Coordinate, Box<dyn std::error::Error>> {
//     let chars: Vec<&str> = player_input.trim().split(',').collect();

//     match chars.len() {
//         2 => {
//             let x = chars[0].parse::<u32>()?; // ?: if successful, unwrap the integer value; Otherwise, return immediately
//             let y = chars[1].parse::<u32>()?;

//             Ok(Coordinate{x: x, y: y})
//         },
//         _ => Err("Expected exactly two comma-separated integers".into())
//     }
// }

pub fn parse_action(player_input: String) -> Result<Action, Box<dyn std::error::Error>> {
    match player_input.trim() {
        "Reveal" => Ok(Action::Reveal),
        "Flag" => Ok(Action::Flag),
        "Unflag" => Ok(Action::Unflag),
        _ => Err("Wrong action command".into())
    }
}

pub fn get_board_size() -> io::Result<(u32, u32)> {
    println!("Enter your board size: hsize,vsize");

    loop {
        let mut player_input = String::new();
        io::stdin().read_line(&mut player_input)?;

        match parse_input(player_input) {
            Ok((hsize, vsize)) => match validate_board_size(hsize, vsize) {
                Ok(size) => return Ok(size),
                Err(size_error) =>  match size_error {
                    InvalidValue::MaxExceeded => {println!("board too big");},
                    _ => {panic!("should not be here!");}
                }
            },
            Err(parse_error) => match parse_error {
                ParseError::BadFormat => {println!("Expected exactly two comma-separated integers");},
                ParseError::NotNumber(_) => {println!("Not number");}
            }
        }
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
        
        match parse_input(player_input) {
            Ok(coord) => {
                match game.validate_coordinate(&coord) {
                    Ok(coord) => return Ok(coord), // all match arms return ()
                    Err(value_error) => match value_error {
                        InvalidValue::OutOfBounds => {println!("coordinate out of bounds");},
                        InvalidValue::Unavailable => {println!("tile at {:?} already revealed", coord)},
                        _ => {panic!("should not be here!")}
                    }
                }
            }, 
            Err(parse_error) => match parse_error {
                ParseError::BadFormat => {println!("Expected exactly two comma-separated integers");},
                ParseError::NotNumber(_) => {println!("Not number");}
            }
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
