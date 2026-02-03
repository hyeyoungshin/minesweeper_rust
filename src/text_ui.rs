use crate::game::*;
use crate::game::player::*;
use crate::game::board::*;
use std::io;
use std::fmt;

macro_rules! try_again {
    ($e: expr) => {{
        println!("{}. Try again.", $e);
        continue;
    }};
}

pub const BOARD_MAX_SIZE: u32 = 30;

#[derive(Debug)]
pub enum ParseErr {
    BadFormat,
    NotNum,
    NegativeNum,
}

#[derive(Debug)]
pub enum CoordinateErr {
    OutOfBounds,
    TileRevealed,
    TileFlagged,
}

#[derive(Debug)]
pub enum InvalidErr {
    InvalidAction,
    InvalidCoordinate(CoordinateErr),
    InvalidBoardSize,
}

impl fmt::Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseErr::BadFormat => write!(f, "Expected exactly two comma-separated positive number"),
            ParseErr::NotNum => write!(f, "Not a number"),
            ParseErr::NegativeNum => write!(f, "Negative number"),
        }
    }
}

impl fmt::Display for InvalidErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InvalidErr::InvalidAction => write!(f, "Invalid action"),
            InvalidErr::InvalidBoardSize => write!(f, "Invalid board size"),
            InvalidErr::InvalidCoordinate(coordinate_err) => write!(f, "Invalid coordinate: {}", coordinate_err),
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
pub fn get_coordinate(game: &Game, player: &Player) -> io::Result<Coordinate> {
    println!("Enter a coordinate: x,y");
    // The loop continues until one branch hits return Ok(valid_coord)
    loop {
        let mut player_input = String::new();
        io::stdin().read_line(&mut player_input)?;

        let parsed_coord = match parse_coordinate(&player_input) {
            Ok(coord) => coord,
            Err(e) => {
                try_again!(e);
            }
        };
         
        match game.validate_coordinate(&parsed_coord, player) {
            Ok(coord) => return Ok(coord),
            Err(e) => { 
                try_again!(e);
            }
        }
    }
}

// Parses player's inputs from the console
// For example, 
//   2,3 is ok
//   4,k is error - not number
//   1,2,3 is error - bad format
pub fn parse_coordinate(player_input: &String) -> Result<Coordinate, ParseErr> {
    let chars: Vec<&str> = player_input.trim().split(',').collect();

    match chars.len() {
        2 => {
            let x = chars[0].parse::<i32>()
                .map_err(|_| ParseErr::NotNum)?;
            let y = chars[1].parse::<i32>()
                .map_err(|_| ParseErr::NotNum)?;

            if x >= 0 && y >= 0 {
                Ok(Coordinate{ x: x as u32, y: y as u32 })
            } else {
                Err(ParseErr::NegativeNum)
            }
        },
        _ => Err(ParseErr::BadFormat)
    }
}

pub fn get_action(game: &Game, player: &Player, coordinate: Coordinate) -> io::Result<PlayerAction> {
    println!("Enter an action: Flag, Unflag, or Reveal");

    loop {
        let mut player_input = String::new();
        io::stdin().read_line(&mut player_input)?;
        
        let parsed_action = match parse_action(player_input) {
            Ok(action) => action,
            Err(parse_err) => {
                try_again!(parse_err);
            }
        };

        let player_action = PlayerAction{ player_id: player.id, coordinate, action: parsed_action };

        match game.validate_action(player_action, &coordinate) {
                Ok(player_action) => return Ok(player_action),
                Err(invalid_err) => { 
                    try_again!(invalid_err) 
                }
            }
    }
}

pub fn parse_action(player_input: String) -> Result<Action, ParseErr> {
    match player_input.trim() {
        "Reveal" => Ok(Action::Reveal),
        "Flag" => Ok(Action::Flag),
        "Unflag" => Ok(Action::Unflag),
        _ => Err(ParseErr::BadFormat)
    }
}

pub type BoardSize = (u32, u32);

pub fn get_board_size() -> io::Result<BoardSize> {
    println!("Enter your board size: h_size, v_size");

    loop {
        let mut player_input = String::new();
        io::stdin().read_line(&mut player_input)?;

        let parsed_board_size = match parse_board_size(player_input) {
            Ok(board_size) => board_size,
            Err(parse_err) => {
                try_again!(parse_err);
            }
        };

        match Board::validate_size(parsed_board_size.0, parsed_board_size.1) {
            Ok(board_size) => return Ok(board_size),
            Err(size_err) => {
                try_again!(size_err);
            }
        }
    }         
}

pub fn parse_board_size(player_input: String) -> Result<BoardSize, ParseErr> {
    let chars: Vec<&str> = player_input.trim().split(',').collect();

    match chars.len() {
        2 => {
            let x = chars[0].parse::<i32>()
                .map_err(|_| ParseErr::NotNum)?;
            let y = chars[1].parse::<i32>()
                .map_err(|_| ParseErr::NotNum)?;

            if x > 0 && y > 0 {
                Ok((x as u32, y as u32))
            } else {
                Err(ParseErr::NegativeNum)
            }
        },
        _ => Err(ParseErr::BadFormat)
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
            Err(parse_err) => {
                try_again!(parse_err);
            }
        }
    }
}

pub fn parse_difficulty(player_input: String) -> Result<Difficulty, ParseErr> {
    match player_input.trim() {
        "Easy" => Ok(Difficulty::Easy),
        "Medium" => Ok(Difficulty::Medium),
        "Hard" => Ok(Difficulty::Hard),
        _ => Err(ParseErr::BadFormat)
    }
}
