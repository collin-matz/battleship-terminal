/// This module contains logic for managing board state.
use std::{fmt, vec};
use colored::Colorize;
use crate::game::components::ship::ShipType;

use super::ship;


/// Constants for controlling the size of the game board.
pub const ROWS: usize = 10;
pub const COLS: usize = 10;

/// An enum that defines all possible states a board cell can exist in.
/// When a cell is modified on the board, we simply adjust the enumeration
/// assigned to that cell.
#[derive(Clone, Copy)]
pub enum CellState {
    Empty,
    Guessed,
    OwnShip(ship::ShipType),
    HitShip,
    Highlighted,
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cell_content = match self {
            CellState::Empty => "□".black(),
            CellState::Guessed => "▣".white(),
            CellState::OwnShip(ship_type) => {
                match ship_type {
                    ship::ShipType::Carrier(size, symbol)
                    | ship::ShipType::Battleship(size, symbol)
                    | ship::ShipType::Destroyer(size, symbol)
                    | ship::ShipType::Submarine(size, symbol)
                    | ship::ShipType::PatrolBoat(size, symbol) => {
                        format!("{}", symbol).green()
                    }
                }
            }
            CellState::HitShip => "◼".red(),
            CellState::Highlighted => "◼".blue(),
        };
        write!(f, "{}", cell_content)
    }
}

pub struct Cell {
    state: CellState,
    prev_state: CellState
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.state)
    }
}

impl Cell {
    /// Create a new empty cell.
    pub fn new() -> Self {
        Self { state: CellState::Empty, prev_state: CellState::Empty }
    }

    /// Apply the highlight state to a cell. Set the previous state that way we can undo
    /// the highlight properly.
    pub fn highlight(&mut self) {
        self.prev_state = self.state;
        self.state = CellState::Highlighted;
    }

    /// Undo the highlight state to a cell
    pub fn undo_highlight(&mut self) {
        self.state = self.prev_state;
    }

    pub fn get_state(&self) -> CellState {
        self.state
    }
}

/// A structure for encapsulating board state and logic.
pub struct Board {
    cells: vec::Vec<Cell>
}

impl Board {
    /// Generate a new board of empty cells.
    pub fn default() -> Self {
        let mut cells: vec::Vec<Cell> = vec![];
        for _ in 0..(ROWS*COLS) {
            cells.push(Cell::new());
        }
        Self { cells }
    }

    /// Set a cell in the board to the specified new state.
    pub fn set(&mut self, row: usize, col: usize, state: CellState) {
        self.update(row, col, state);
    }

    /// Given a row and column index, return a reference to the Cell at that position.
    pub fn get(&self, row: usize, col: usize) -> &Cell {
        &self.cells[row * COLS + col]
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut Cell {
        &mut self.cells[row * COLS + col]
    }

    pub fn highlight_cell(&mut self, row: usize, col: usize) {
        self.get_mut(row, col).highlight();
    }

    pub fn undo_highlight_cell(&mut self, row: usize, col: usize) {
        self.get_mut(row, col).undo_highlight();
    }

    /// Updates a single cell of the board.
    fn update(&mut self, row: usize, col: usize, state: CellState) {
        // due to type restrictions, we do not need to check if row and
        // col are > 0
        if (row < ROWS) && (col < COLS) {
            self.cells[row * col + col].state = state;
        }
    }
}
