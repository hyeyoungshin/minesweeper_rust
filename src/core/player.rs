use crate::core::board::Coordinate;

use std::sync::atomic::{AtomicU32, Ordering};

static NEXT_PLAYER_ID: AtomicU32 = AtomicU32::new(1);

pub type PlayerId = u32;

#[derive(Debug)]
pub struct PlayerAction {
    pub player_id: PlayerId,
    pub coordinate: Coordinate,
    pub action: Action, 
}

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub points: i32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Action{
    Reveal, // Revealing all in the hint = 0 case is always 3. Revealing a hint tile is 1.
    Flag,   // If flagged a non-mine tile, it reveals. In this case, even if the tile has hint = 0, it does not reveal all of its neighbors. The player gets a penalty point -1.
}

impl Player {
    pub fn new(name: String) -> Self {
        Player { 
            id: NEXT_PLAYER_ID.fetch_add(1, Ordering::Relaxed), 
            name: name,
            points: 0 
        }
    }

    #[cfg(test)]
    pub fn new_with_id(id: PlayerId, name: &str) -> Self {
        Player { id, name: name.to_string(), points: 0 }
    }

    pub fn add_points(&self, points: i32) -> Self {
        Player { id: self.id, name: self.name.clone() , points: self.points + points }
    }

    pub fn subtract_points(&self, points: i32) -> Self {
        Player { id: self.id, name: self.name.clone() , points: self.points - points }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_id() {
        let p1 = Player::new("alice".to_string());
        let p2 = Player::new("bob".to_string());
        let p3 = Player::new("carol".to_string());

        assert!(p2.id > p1.id);
        assert!(p3.id > p2.id);

    }

    #[test]
    fn two_players() {
        let new_player = Player::new("hyeyoung".to_string());
        let newer_player = Player::new("william".to_string());
        assert_ne!(new_player.id, newer_player.id);
    }

    #[test]
    fn test_many_players_have_unique_ids() {
        let players: Vec<_> = (0..100)
            .map(|i| Player::new(format!("player{}", i)))
            .collect();
        
        let ids: std::collections::HashSet<_> = players.iter().map(|p| p.id).collect();
        
        assert_eq!(ids.len(), 100); 
    }
}
