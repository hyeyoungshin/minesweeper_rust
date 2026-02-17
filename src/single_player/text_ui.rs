use crate::core::board::{Coordinate};

use crate::core::player::{Player, Action, PlayerAction};
use crate::core::validation::{InvalidErr, CoordinateErr};
use crate::core::validation::*;

use crate::core::game::*;
use crate::core::game::{Game, Difficulty};

use std::io;
use std::fmt;

macro_rules! try_again {
    ($e: expr) => {{
        println!("{}. Try again.", $e);
        continue;
    }};
}

#[derive(Debug)]
pub enum ParseErr {
    ParsingFailed,
    NotNum,
    NegativeNum,
}

impl fmt::Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseErr::ParsingFailed => write!(f, "Failed parsing input"),
            ParseErr::NotNum => write!(f, "Not a number"),
            ParseErr::NegativeNum => write!(f, "Negative number"),
        }
    }
}

impl fmt::Display for InvalidErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InvalidErr::InvalidAction => write!(f, "Invalid action"),
            InvalidErr::InvalidPlayer => write!(f, "Invalid player"),
            InvalidErr::InvalidSize => write!(f, "Invalid size"),
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

// TODO: a lot of unwrapping happening here
pub fn start_game() -> Game {
    println!("Let's play minesweeper game!");

    let num_players = get_num_players().unwrap();

    let players: Vec<Player> = (0..num_players).into_iter()
      .map(|_| Player::new(get_name()))
      .collect();

    let (h_size, v_size) = get_board_size().unwrap();
    let game_level = get_difficulty().unwrap();

    let mut game = Game::new(h_size, v_size, game_level);
    
    game = players.into_iter()
      .fold(game,|game, player| {
        game.add_player(player)
      });
    
    game.board.print();
    
    println!("number of mines: {}\n", game.board.num_mines());
    game
}

pub fn print_scores(game: &Game) {
    game.players.values()
      .for_each(|player| println!("{}: {}", player.name, player.points));
    
    println!();
}

// Prints the end of game message
pub fn announce_winners(game: &Game) {
    game.get_winners().into_iter()
      .for_each(|winner| print!("{} ", winner.name));

    println!("won!");    
}

// Prompts a message to get a valid coordinate from player
pub fn get_coordinate(game: &Game, player: &Player) -> io::Result<Coordinate> {
    println!("{}, enter a coordinate: x,y", player.name);
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
         
        match validate_coordinate(&game.board, &parsed_coord, player) {
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
                .map_err(|_| ParseErr::ParsingFailed)?;
            let y = chars[1].parse::<i32>()
                .map_err(|_| ParseErr::ParsingFailed)?;

            if x >= 0 && y >= 0 {
                Ok(Coordinate{ x: x as u32, y: y as u32 })
            } else {
                Err(ParseErr::NegativeNum)
            }
        },
        _ => Err(ParseErr::ParsingFailed)
    }
}

pub fn get_action(game: &Game, player: &Player, coordinate: Coordinate) -> io::Result<PlayerAction> {
    println!("Enter an action: Flag or Reveal");

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

        match validate_action(&game, player_action, &coordinate) {
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
        // "Unflag" => Ok(Action::Unflag),
        _ => Err(ParseErr::ParsingFailed)
    }
}

const MAX_NUM_PLAYERS: u32 = 5;

pub fn get_num_players() -> io::Result<u32> {
    println!("How many players?");

    loop {
        let mut player_input = String::new();
        io::stdin().read_line(&mut player_input)?;

        let num_players = player_input.trim().parse::<u32>();

        match num_players {
            Ok(num_players) => if num_players > MAX_NUM_PLAYERS {
                try_again!(InvalidErr::InvalidSize);
            } else {
                return Ok(num_players)
            },
            Err(_) => {
                try_again!(ParseErr::ParsingFailed);
            }
        }
    }
}

pub type BoardSize = (u32, u32);

pub fn get_board_size() -> io::Result<BoardSize> {
    println!("Enter your board size: n,n");

    loop {
        let mut player_input = String::new();
        io::stdin().read_line(&mut player_input)?;

        let parsed_board_size = match parse_board_size(player_input) {
            Ok(board_size) => board_size,
            Err(parse_err) => {
                try_again!(parse_err);
            }
        };

        match validate_board_size(parsed_board_size.0, parsed_board_size.1) {
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
        _ => Err(ParseErr::ParsingFailed)
    }
}

pub fn get_name() -> String {
    println!("Enter your name");

    let mut player_input = String::new();
    io::stdin()
        .read_line(&mut player_input)
        .expect("Failed to read name");
    
    player_input.trim().to_string()
}

pub fn get_difficulty() -> io::Result<Difficulty> {
    println!("Enter the level of difficulty: Easy, Medium, or Hard");

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
        _ => Err(ParseErr::ParsingFailed)
    }
}
