use crate::game::*;
use crate::game::board::*;
use crate::game::player::*;

use std::io;

#[derive(Debug)]
pub enum ParseError {
    BadFormat,
    NotNumber(std::num::ParseIntError),
}

pub enum ValidationError {
    OutOfBounds,
    TileRevealed,
    MaxExceeded,
    NegativeSize
}

pub trait FromPair {
    fn from_pair(x: i32, y: i32) -> Self; // x, y are i32 to handle negative case in validation, not parsing
}

impl FromPair for Coordinate {
    fn from_pair(x: i32, y: i32) -> Self {
        Coordinate { x: x as u32, y: y as u32 } // non-negative i32 to u32 is ok!
    }
}

impl FromPair for (i32, i32) {
    fn from_pair(x: i32, y: i32) -> Self {
        (x, y)
    }
}

pub fn parse_input<T: FromPair> (player_input: String) -> Result<T, ParseError> {
    let chars: Vec<&str> = player_input.trim().split(',').collect();
    // TODO: -1,n triggers not number parse error
    match chars.len() {
        2 => {
            let x = chars[0].parse::<i32>()
                .map_err(|e| ParseError::NotNumber(e))?;
            let y = chars[1].parse::<i32>()
                .map_err(|e| ParseError::NotNumber(e))?;

            Ok(T::from_pair(x, y))
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
        io::stdin().read_line(&mut player_input)?;

        match parse_input(player_input) {
            Ok((hsize, vsize)) => match Board::validate_size(hsize, vsize) {
                Ok(size) => return Ok(size),
                Err(size_error) =>  match size_error {
                    ValidationError::MaxExceeded => {println!("board too big");},
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
         
        match parse_input(player_input) {
            Ok(coord) => {
                match game.board.validate_coordinate(&coord) {
                    Ok(coord) => return Ok(coord), // all match arms return ()
                    Err(value_error) => match value_error {
                        ValidationError::OutOfBounds => { println!("coordinate out of bounds"); },
                        ValidationError::TileRevealed => { println!("tile at {:?} already revealed", coord) },
                        _ => { panic!("should not be here!") }
                    }
                }
            }, 
            Err(parse_error) => match parse_error {
                ParseError::BadFormat => { println!("Expected exactly two comma-separated integers"); },
                ParseError::NotNumber(_) => { println!("Not number"); }
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
            Err(msg) => println!("{}", msg),
        }
    }
}

pub fn get_id() -> io::Result<String> {
    println!("Enter your id");

    let mut player_input = String::new();
    io::stdin().read_line(&mut player_input)?; // catches erros from OS        
    
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

pub fn start_game() {
    println!("Let's play minesweeper game!");
}

pub fn end_game(game: &Game) {
    match game.status {
        GameStatus::Win => println!("You won!"),
        GameStatus::Over => println!("You lost..."),
        _ => {panic!("should not be here");}
    }
} 