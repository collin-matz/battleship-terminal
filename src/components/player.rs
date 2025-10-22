/// This module contains logic for managing player state.
use super::board;
use super::ship;

use std::vec;

/// A struct for encapsulating player logic and state.
pub struct Player {
    name: &'static str,
    board: board::Board,
    cursor: (u8, u8),
    ships: vec::Vec<ship::Ship>
}

impl Player {
    /// Create a new player with the given name.
    pub fn new(name: &'static str) -> Self {
        Self {
            name: name,
            board: board::Board::new_empty(),
            cursor: (0, 0),
            ships: ship::Ship::default()
        }
    }

    pub fn get_ships(&self) -> &vec::Vec<ship::Ship> {
        &self.ships
    }
}
