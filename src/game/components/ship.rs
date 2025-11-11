/// This module contains logic for managing and creating ships.
use std::{fmt, vec};
use rand::distributions::{Distribution, Standard};
use super::board;


/// An enum to represent the orientation of a ship.
pub enum ShipOrientation {
    Left,
    Up,
    Right,
    Down
}

impl ShipOrientation {
    /// Given the current ship orientation, return the next orientation in clockwise order.
    pub fn next(&self) -> ShipOrientation {
        match self {
            ShipOrientation::Left => ShipOrientation::Up,
            ShipOrientation::Up => ShipOrientation::Right,
            ShipOrientation::Right => ShipOrientation::Down,
            ShipOrientation::Down => ShipOrientation::Left,
        }
    }
}

/// Implement the rand::Distribution trait for ShipOrientation in order to randomly
/// select ship orientations for automatic board setup.
impl Distribution<ShipOrientation> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ShipOrientation {
        match rng.gen_range(0..4) {
            0 => ShipOrientation::Left,
            1 => ShipOrientation::Up,
            2 => ShipOrientation::Right,
            _ => ShipOrientation::Down,
        }
    }
}

/// An enum that defines all possible ship types for the game.
#[derive(Clone, Copy, PartialEq)]
pub enum ShipType {
    Carrier(usize, char),
    Battleship(usize, char),
    Destroyer(usize, char),
    Submarine(usize, char),
    PatrolBoat(usize, char),
}

impl fmt::Display for ShipType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShipType::Carrier(_, _) => write!(f, "Carrier"),
            ShipType::Battleship(_, _) => write!(f, "Battleship"),
            ShipType::Destroyer(_, _) => write!(f, "Destroyer"),
            ShipType::Submarine(_, _) => write!(f, "Submarine"),
            ShipType::PatrolBoat (_, _)=> write!(f, "Patrol Boat")
        }
    }
}

impl ShipType {
    // a static array to all possible
    // ship types. this allows us to iterate over
    // whenever we need to have a reference to all
    // the possible ships
    pub const ALL: [ShipType; 5] = [
        ShipType::Carrier(5, 'C'),
        ShipType::Battleship(4, 'B'),
        ShipType::Destroyer(3, 'D'),
        ShipType::Submarine(3, 'S'),
        ShipType::PatrolBoat(2, 'P')
    ];

    /// Generate a consuming iterator over the ShipType options
    pub fn iter() -> impl Iterator<Item = ShipType> {
        Self::ALL.into_iter()
    }

    /// Get the size of this ship.
    pub fn size(&self) -> usize {
        match self {
            ShipType::Carrier(size, _)
            | ShipType::Battleship(size, _)
            | ShipType::Destroyer(size, _)
            | ShipType::Submarine(size, _)
            | ShipType::PatrolBoat(size, _) => *size,
        }
    }

    /// Get the display symbol for this ship.
    pub fn symbol(&self) -> char {
        match self {
            ShipType::Carrier(_, symbol)
            | ShipType::Battleship(_, symbol)
            | ShipType::Destroyer(_, symbol)
            | ShipType::Submarine(_, symbol)
            | ShipType::PatrolBoat(_, symbol) => *symbol,
        }
    }
}

/// A struct to contain all associated data with a ship. 
pub struct Ship {
    cells: vec::Vec<(usize, usize)>
}

impl Ship {
    /// Return a new ship structure with an empty vector of owned cells.
    pub fn new(cells: vec::Vec<(usize, usize)>) -> Self {
        Self { cells: cells }
    }

    /// Check whether this ship is sunk based on the current board state.
    pub fn is_sunk(&self, board: &board::Board) -> bool {
        // check each corresponding cell in the board to see if it's been hit
        for (row, col) in self.cells.iter() {
            let cell: &board::Cell = board.get(*row, *col);
            if cell.get_state() != board::CellState::HitShip {
                return false;
            }
        }
        true
    }
}
