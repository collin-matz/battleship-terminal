/// This module contains logic for managing player state.
use std::vec;
use rand::{self, Rng};
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

    pub fn get_hidden_cell(&self, row: usize, col: usize) -> board::Cell {
        let cell: &board::Cell = self.board.get(row, col);
        match cell.get_state() {
            board::CellState::OwnShip(_) => board::Cell::get_hidden_cell(cell),
            _ => cell.clone()
        }
    }

    pub fn get_cell_mut(&mut self, row: usize, col: usize) -> &mut board::Cell {
        self.board.get_mut(row, col)
    }

    /// Set a ship on the player's board.
    pub fn add_ship(&mut self, cell_indices: vec::Vec<(usize, usize)>, ship_type: ship::ShipType) {
        let ship: ship::Ship = ship::Ship::new(cell_indices.clone());
        self.ships.push(ship);
        // update the board cells to reflect the ship placement
        for (row, col) in cell_indices {
            self.board.set(row, col, board::CellState::OwnShip(ship_type))
        }
    }

    /// Returns true if all of the player's ships are sunk.
    pub fn all_ships_sunk(&self) -> bool {
        for ship in self.ships.iter() {
            if !ship.is_sunk(&self.board) {
                return false;
            }
        }
        true
    }

    /// Apply a guess to the player's board and return the resulting cell state.
    pub fn guess(&mut self, row: usize, col: usize) {
        let cell: &mut board::Cell = self.get_cell_mut(row, col);
        match cell.get_prev_state() {
            board::CellState::OwnShip(_) => {
                self.board.update(row, col, board::CellState::HitShip);
            },
            board::CellState::Empty => {
                self.board.update(row, col, board::CellState::Guessed);
            },
            _ => {} // do nothing for other cell states
        }
    }

    pub fn auto_guess(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let r = rng.gen_range(0..board::ROWS);
            let c = rng.gen_range(0..board::COLS);
            let cell: &board::Cell = self.get_cell(r, c);
            match cell.get_state() {
                board::CellState::HitShip | board::CellState::Guessed => {
                    continue; // already guessed, try again
                },
                _ => {
                    // valid guess
                    self.guess(r, c);
                    break;
                }
            }
        }
    }

    /// Automatically place all ships for the player. This is used for
    /// computer players / players who want to randomly setup their boards.
    pub fn auto_place_ships(
        &mut self,
        max_tries_per_ship: usize, 
        max_global_restarts: usize
    ) -> Result<(), ()>{
        // we use a simple retry algorithm that keeps trying to place ships
        // until all ships are placed successfully
        let mut rng = rand::thread_rng();

        for _ in 0..max_global_restarts {
            // reset the board to be a default empty board
            self.ships.clear();
            self.board = board::Board::default();

            for ship_type in ship::ShipType::ALL.iter() {

                let mut placed: bool = false;
                for _ in 0..max_tries_per_ship {
                    let orient: ship::ShipOrientation = rand::random();
                    let r = rng.gen_range(0..board::ROWS);
                    let c = rng.gen_range(0..board::COLS);

                    let indices: Option<vec::Vec<(usize, usize)>> = self.board.try_place_ship(r, c, orient, *ship_type);
                    if let Some(cell_indices) = indices {
                        self.add_ship(cell_indices, *ship_type);
                        placed = true;
                        break;
                    } 
                }

                if !placed {
                    continue; // restart the global placement process
                }
            }
            return Ok(()); 
        }

        Err(())
    }
}
