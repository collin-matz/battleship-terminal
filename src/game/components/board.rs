/// This module contains logic for managing board state.
use std::{fmt, vec};
use colored::Colorize;
use super::ship;


/// Constants for controlling the size of the game board.
pub const ROWS: usize = 10;
pub const COLS: usize = 10;

/// An enum that defines all possible states a board cell can exist in.
/// When a cell is modified on the board, we simply adjust the enumeration
/// assigned to that cell.
#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Empty,
    Guessed,
    OwnShip(ship::ShipType),
    HitShip,
    Highlighted,
    InvalidPlacement,
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cell_content = match self {
            CellState::Empty => "□".black(),
            CellState::Guessed => "▣".white(),
            CellState::OwnShip(ship_type) => {
                match ship_type {
                    ship::ShipType::Carrier(_, symbol)
                    | ship::ShipType::Battleship(_, symbol)
                    | ship::ShipType::Destroyer(_, symbol)
                    | ship::ShipType::Submarine(_, symbol)
                    | ship::ShipType::PatrolBoat(_, symbol) => {
                        format!("{}", symbol).green()
                    }
                }
            }
            CellState::HitShip => "◼".red(),
            CellState::Highlighted => "◼".blue(),
            CellState::InvalidPlacement => "X".red(),
        };
        write!(f, "{}", cell_content)
    }
}

#[derive(Clone)]
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

    /// Undo the previous cell state change.
    pub fn undo(&mut self) {
        self.state = self.prev_state;
    }

    /// Apply the highlight state to a cell. Set the previous state that way we can undo
    /// the highlight properly.
    pub fn highlight(&mut self) {
        self.prev_state = self.state;
        self.state = CellState::Highlighted;
    }

    /// Apply the invalid placement state to a cell. Set the previous state that way we can undo
    /// the invalid placement properly.
    pub fn invalidate(&mut self) {
        self.prev_state = self.state;
        self.state = CellState::InvalidPlacement;
    }

    /// Get the current state of the cell.
    pub fn get_state(&self) -> CellState {
        self.state
    }

    /// Get the previous state of the cell.
    pub fn get_prev_state(&self) -> CellState {
        self.prev_state
    }

    pub fn get_hidden_cell(cell: &Cell) -> Cell {
        match cell.state {
            CellState::OwnShip(_) => Cell { state: CellState::Empty, prev_state: CellState::Empty },
            _ => Cell { state: cell.state, prev_state: cell.prev_state }
        }
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

    pub fn undo(&mut self, row: usize, col: usize) {
        self.get_mut(row, col).undo();
    }

    /// Updates a single cell of the board.
    pub fn update(&mut self, row: usize, col: usize, state: CellState) {
        // due to type restrictions, we do not need to check if row and
        // col are > 0
        if (row < ROWS) && (col < COLS) {
            self.cells[row * COLS + col].state = state;
            self.cells[row * COLS + col].prev_state = state;
        }
    }

    /// Checks whether a ship placement is valid.
    pub fn try_place_ship(
        &self, 
        r: usize, 
        c: usize, 
        orient: ship::ShipOrientation, 
        ship_type: ship::ShipType
    ) -> Option<vec::Vec<(usize, usize)>> {
        // starting from the given cell, check if the ship can fit in the given orientation
        // without overlapping any existing ships or going out of bounds
        let ship_length: usize = ship_type.size();
        let mut indices: Option<vec::Vec<(usize, usize)>> = Some(vec![]);

        for i in 0..ship_length {
            match orient {
                ship::ShipOrientation::Up => {
                    if (i > r) || self.get(r - i, c).get_state() != CellState::Empty {
                        indices = None;
                        break;
                    }
                    indices.as_mut().unwrap().push((r - i, c));
                },
                ship::ShipOrientation::Down => {
                    if (i + r >= ROWS) || self.get(r + i, c).get_state() != CellState::Empty {
                        indices = None;
                        break;
                    }
                    indices.as_mut().unwrap().push((r + i, c));
                },
                ship::ShipOrientation::Left => {
                    if (i > c) || self.get(r, c - i).get_state() != CellState::Empty {
                        indices = None;
                        break;
                    }
                    indices.as_mut().unwrap().push((r, c - i));
                },
                ship::ShipOrientation::Right => {
                    if (i + c >= COLS) || self.get(r, c + i).get_state() != CellState::Empty {
                        indices = None;
                        break;
                    }
                    indices.as_mut().unwrap().push((r, c + i));
                }
            }
        };

        indices
    }   
}
