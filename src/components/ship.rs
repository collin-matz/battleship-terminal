/// This module contains logic for managing and creating ships.
use std::{fmt, vec};

/// An enum that defines all possible ship types for the game.
#[derive(Clone, Copy)]
pub enum ShipType {
    Carrier,
    Battleship,
    Destroyer,
    Submarine,
    PatrolBoat,
}

impl fmt::Display for ShipType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShipType::Carrier => write!(f, "Carrier"),
            ShipType::Battleship => write!(f, "Battleship"),
            ShipType::Destroyer => write!(f, "Destroyer"),
            ShipType::Submarine => write!(f, "Submarine"),
            ShipType::PatrolBoat => write!(f, "Patrol Boat")
        }
    }
}

/// A struct to contain all associated data with a ship. 
pub struct Ship {
    ship_type: ShipType, 
    length: u8,
    cells_hit: u8
}

impl Ship {
    /// Given a ship type, return a new ship structure with that ship type
    /// and the size of the ship.
    pub fn new(ship_type_selection: ShipType) -> Self {
        let (ship_type, length) = match ship_type_selection {
            ShipType::Carrier => (ShipType::Carrier, 5),
            ShipType::Battleship => (ShipType::Battleship, 4),
            ShipType::Destroyer => (ShipType::Destroyer, 3),
            ShipType::Submarine => (ShipType::Submarine, 3),
            ShipType::PatrolBoat => (ShipType::PatrolBoat, 2)
        };

        Self { 
            ship_type: ship_type,
            length: length,
            cells_hit: 0
        }
    }

    /// Returns a defaulted list of ships.
    pub fn default() -> vec::Vec<Ship> {
        vec![
            Ship::new(ShipType::Carrier),
            Ship::new(ShipType::Battleship),
            Ship::new(ShipType::Destroyer),
            Ship::new(ShipType::Submarine),
            Ship::new(ShipType::PatrolBoat)
        ]
    }

    pub fn is_sunk(&self) -> bool {
        self.cells_hit == (self.length - 1)
    }

    pub fn get_ship_type(&self) -> ShipType {
        self.ship_type
    }
}
