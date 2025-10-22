/// This module contains logic for managing board state.
use colored::Colorize;
use std::fmt;

/// Constants for controlling the size of the game board.
pub const ROWS: usize = 10;
pub const COLS: usize = 10;

/// An enum that defines all possible states a board cell can exist in.
/// When a cell is modified on the board, we simply adjust the enumeration
/// assigned to that cell.
#[derive(Clone, Copy)]
pub enum Cell {
    Empty,
    Guessed,
    OwnShip,
    OwnShipHit,
    EnemyShipHit,
    Highlighted
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cell_content = match self {
            Cell::Empty => "□".black(),
            Cell::Guessed => "▣".white(),
            Cell::OwnShip => "◼".green(),
            Cell::OwnShipHit => "◼".yellow(),
            Cell::EnemyShipHit => "◼".red(),
            Cell::Highlighted => "◼".blue(),
        };
        write!(f, "{}", cell_content)
    }
}

/// A structure for encapsulating board state and logic.
pub struct Board {
    cells: [Cell; ROWS * COLS]
}

impl Board {
    /// Generate a new board of empty cells.
    pub fn new_empty() -> Self {
        Self {
            cells: [Cell::Empty; ROWS * COLS]
        }
    }

    /// Update a single cell in the board.
    pub fn update(&mut self, row_idx: usize, col_idx: usize, new_state: Cell) {
        if (row_idx < ROWS) || (col_idx < COLS) {
            self.cells[row_idx * col_idx + col_idx] = new_state;
        }
    }

    /// Given a row and column index, return the Cell enum at that position.
    pub fn get(&self, r: usize, c: usize) -> Cell {
        self.cells[r * c + c]
    }
}
