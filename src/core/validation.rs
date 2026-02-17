// Shared validation logic
use crate::single_player::text_ui::BoardSize;

use crate::core::player::{Player, PlayerAction, Action};
use crate::core::board::{Board, Coordinate, TileStatus};
use crate::core::game::{Game};

#[derive(Debug)]
pub enum CoordinateErr {
    OutOfBounds,
    TileRevealed,
    TileFlagged,
}

#[derive(Debug)]
pub enum InvalidErr {
    InvalidAction,
    InvalidPlayer,
    InvalidCoordinate(CoordinateErr),
    InvalidSize,
}

pub const BOARD_MAX_SIZE: u32 = 30; // for single_player mode

pub fn validate_board_size(h_size: u32, v_size: u32) -> Result<BoardSize, InvalidErr> {
    if h_size > BOARD_MAX_SIZE && v_size > BOARD_MAX_SIZE {
        Err(InvalidErr::InvalidSize)
    } else {
        Ok((h_size as u32, v_size as u32))
    }
}

// This function validates player's chosen action for the tile at the coordinate
pub fn validate_action(game: &Game, player_action: PlayerAction, coordinate: &Coordinate) -> Result<PlayerAction, InvalidErr> {
    let tile_status = game.board.get_tile(coordinate);
    let action = player_action.action;

    match (tile_status, action) {
        (TileStatus::Hidden, Action::Flag | Action::Reveal) => Ok(player_action),
         _ => Err(InvalidErr::InvalidAction),
    }
}

// This function validates player's chosen coordinate 
pub fn validate_coordinate(board: &Board, coordinate: &Coordinate, player: &Player) -> Result<Coordinate, InvalidErr> {        
    if board.within_bounds(&(coordinate.x as i32, coordinate.y as i32)) {
        let tile_status = board.get_tile(coordinate);

        match tile_status {
            TileStatus::Revealed(_) => Err(InvalidErr::InvalidCoordinate(CoordinateErr::TileRevealed)),
            TileStatus::Flagged(flagged_by) => if flagged_by == &player.id { 
                Ok(*coordinate) 
            } else {
                Err(InvalidErr::InvalidCoordinate(CoordinateErr::TileFlagged))
            },                    
            _ => Ok(*coordinate)
        }
    } else {
        Err(InvalidErr::InvalidCoordinate(CoordinateErr::OutOfBounds))
    }
}