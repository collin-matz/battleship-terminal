/// This module contains logic for managing player state.
use std::vec;
use super::{board, ship};


/// A struct for encapsulating player logic and state.
pub struct Player<'a> {
    name: &'static str,
    board: board::Board,
    ships: vec::Vec<ship::Ship<'a>>,
}

impl<'a> Player<'a> {
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

    // /// Set a ship on the player's board.
    // pub fn add_ship(&'a mut self, cell_indices: vec::Vec<(usize, usize)>, ship_type: ship::ShipType) {

    //     let mut cells: vec::Vec<&'a board::Cell> = vec![];
    //     for (r, c) in cell_indices {
    //         // get a reference to the actual cells from the provided indices
    //         cells.push(self.get_cell(r, c));
    //     }

    //     // create a new ship with this ship type
    //     self.ships.push(ship::Ship::new(ship_type, cells));
    // }
}
