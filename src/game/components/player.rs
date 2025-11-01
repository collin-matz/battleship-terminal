/// This module contains logic for managing player state.
use std::vec;
use super::{board, ship};


/// A struct for encapsulating player logic and state.
pub struct Player {
    name: &'static str,
    board: board::Board,
    ships: vec::Vec<ship::Ship>,
}

impl Player {
    /// Create a new player with the given name and default ships and board layouts.
    pub fn new(name: &'static str) -> Self {
        Self {
            name: name,
            board: board::Board::default(),
            ships: vec![],  // at player creation, they don't have any placed ships yet
        }
    }

    pub fn get_cell(&self, row: usize, col: usize) -> &board::Cell {
        self.board.get(row, col)
    }

    pub fn get_cell_mut(&mut self, row: usize, col: usize) -> &mut board::Cell {
        self.board.get_mut(row, col)
    }

    /// Set a ship on the player's board.
    pub fn add_ship(&mut self, cell_indices: vec::Vec<(usize, usize)>, ship_type: ship::ShipType) {
        let ship: ship::Ship = ship::Ship::new(ship_type, cell_indices.clone());
        self.ships.push(ship);
        // update the board cells to reflect the ship placement
        for (row, col) in cell_indices {
            self.board.set(row, col, board::CellState::OwnShip(ship_type))
        }
    }
}
