use crate::game::*;
use crate::game::board::*;
use crate::game::player::*;

use std::io;
use std::fmt;

#[derive(Debug)]
pub enum ParseErr {
    BadFormat,
    NotNum, //(std::num::ParseIntError),
    NegativeNum
}

// Validation error types
#[derive(Debug)]
pub enum SizeErr {
    MaxExceeded,
    NegativeSize
}

#[derive(Debug)]
pub enum CoordinateErr {
    OutOfBounds,
    TileRevealed,
    TileFlagged
}

const BOARD_MAX_SIZE: u32 = 25;

impl fmt::Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseErr::BadFormat => write!(f, "Expected exactly two comma-separated positive number"),
            ParseErr::NotNum => write!(f, "Not a number"),
            ParseErr::NegativeNum => write!(f, "Negative number")
        }
    }
}

impl fmt::Display for SizeErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SizeErr::MaxExceeded => write!(f, "Max size exceeded"),
            SizeErr::NegativeSize => write!(f, "Board dimensions must be positive"),
        }
    }
}

impl fmt::Display for CoordinateErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CoordinateErr::OutOfBounds => write!(f, "Coordinate out of bounds"),
            CoordinateErr::TileRevealed => write!(f, "Tile is already revealed"),
            CoordinateErr::TileFlagged => write!(f, "Tile is flagged"),
        }
    }
}

// pub trait FromPair {
//     fn from_pair(x: i32, y: i32) -> Self; // x, y are i32 to handle negative case in validation, not parsing
// }

// impl FromPair for Coordinate {
//     fn from_pair(x: i32, y: i32) -> Self {
//         Coordinate { x: x as u32, y: y as u32 } // non-negative i32 to u32 is ok!
//     }
// }

// impl FromPair for (i32, i32) {
//     fn from_pair(x: i32, y: i32) -> Self {
//         (x, y)
//     }
// }

// Prints the welcome message
pub fn start_game() {
    println!("Let's play minesweeper game!");
}

// Prints the end of game message
pub fn end_game(game: &Game) {
    match game.status {
        GameStatus::Win => println!("You won!"),
        GameStatus::Over => println!("You lost..."),
        _ => {panic!("should not be here");}
    }
} 

// Prompts a message to get a valid coordinate from player
pub fn get_coordinate(game: &Game) -> io::Result<Coordinate> {
    println!("Enter a coordinate: x,y");
    // The loop continues until one branch hits return Ok(valid_coord)
    loop {
        let mut player_input = String::new();
        io::stdin().read_line(&mut player_input)?;

        let parsed_coord = match parse_coordinate(&player_input) {
            Ok(coord) => coord,
            Err(e) => {
                println!("{e}. Try again.");
                continue;
            }
        };
         
        match game.board.validate_coordinate(&parsed_coord) {
            Ok(coord) => return Ok(coord),
            Err(e) => println!("{e}. Try again."),
        }
    }
}


// Parses player's inputs from the console
// For example, 
//   2,3 is ok
//   4,k is error - not number
//   1,2,3 is error - bad format
pub fn parse_coordinate(player_input: &String) -> Result<Coordinate, ParseError> {
    let chars: Vec<&str> = player_input.trim().split(',').collect();

    match chars.len() {
        2 => {
            let x = chars[0].parse::<i32>()
                .map_err(|e| ParseError::NotNum(e))?;
            let y = chars[1].parse::<i32>()
                .map_err(|e| ParseError::NotNum(e))?;

            if x > 0 && y > 0 {
                Ok(Coordinate{ x: x as u32, y: y as u32 })
            } else {
                Err(ParseError::NegativeNum)
            }
        },
        _ => Err(ParseError::BadFormat)
    }
}

pub fn parse_action(player_input: String) -> Result<Action, Box<dyn std::error::Error>> {
    match player_input.trim() {
        "Reveal" => Ok(Action::Reveal),
        "Flag" => Ok(Action::Flag),
        _ => Err("Wrong action command".into())
    }
}

pub fn get_board_size() -> io::Result<(u32, u32)> {
    println!("Enter your board size: h_size, v_size");

    loop {
        let mut player_input = String::new();
        io::stdin()
            .read_line(&mut player_input)?;

        match validate_input(player_input) {
            Ok((hsize, vsize)) => match Board::validate_size(hsize, vsize) {
                Ok(size) => return Ok(size),
                Err(ValidationError::MaxExceeded) => { println!("board too big"); },
                Err(e) => { println!("Validation error: {:?}", e); },
                },
            Err(ParseError::BadFormat ) => { println!("Expected exactly two comma-separated integers"); },
            Err(ParseError::NotNumber(_)) => { println!("Not number"); }
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
            Err(msg) => println!("{}", msg),
        }
    }
}

pub fn get_name() -> io::Result<String> {
    println!("Enter your name");

    let mut player_input = String::new();
    io::stdin().read_line(&mut player_input)?;
    
    return Ok(player_input)
}

pub fn get_difficulty() -> io::Result<Difficulty> {
    println!("Enter the level of difficulty: Easy({}), Medium({}), Hard({})", EASY, MEDIUM, HARD);

    loop {
        let mut player_input = String::new();
        io::stdin().read_line(&mut player_input)?;
        
        match parse_difficulty(player_input) {
            Ok(difficulty) => return Ok(difficulty),
            Err(msg) => println!("{}", msg)
        }
    }
}

pub fn parse_difficulty(player_input: String) -> Result<Difficulty, Box<dyn std::error::Error>> {
    match player_input.trim() {
        "Easy" => Ok(Difficulty::Easy),
        "Medium" => Ok(Difficulty::Medium),
        "Hard" => Ok(Difficulty::Hard),
        _ => Err("Parsing failed. Enter the level of difficulty again!".into())
    }
}

