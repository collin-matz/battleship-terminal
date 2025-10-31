/// This module contains logic for managing and creating ships.
use std::{fmt, vec};
use super::board;


/// An enum that defines all possible ship types for the game.
#[derive(Clone, Copy)]
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
pub struct Ship<'a> {
    ship: ShipType,
    cells: vec::Vec<&'a board::Cell>,
    is_sunk: bool
}

impl<'a> Ship<'a> {
    /// Given a ship type, return a new ship structure with that ship type
    /// and an empty vector of owned cells.
    pub fn new(ship: ShipType, cells: vec::Vec<&'a board::Cell>) -> Self {
        Self { 
            ship: ship,
            cells: cells,
            is_sunk: false
        }
    }

    /// A function to check if this ship is sunk.
    pub fn is_sunk(&mut self) -> bool {
        // if this ship is not yet sunk, check if it is.
        // return the value of self.is_sunk
        if !self.is_sunk { 
            self.is_sunk = self.cells.iter().all(
                |cell| if let board::CellState::HitShip = cell.get_state() { true } else { false }
            ); 
        };
        self.is_sunk
    }
}
