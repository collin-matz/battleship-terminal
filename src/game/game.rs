/// Module for housing game logic and management.
use std::vec;
use thiserror::Error;
use crate::game::components::{board, player, ship};


#[derive(Debug, Error)]
pub enum GameError {
    #[error("Game unexpectedly crashed")]
    UnknownError
}

pub struct Game {
    player_a: player::Player,
    player_b: player::Player,
    // we make turn count a 'usize' instead of something like 'u16' or 'u32'
    // because the turn_count is dictated by the board size, and since
    // board size is type 'usize', it would follow that turn_count should be also
    turn_count: usize,
}

impl Game {
    /// Create a new game instance with the two players and a turn count of 0.
    pub fn new(player_a: player::Player, player_b: player::Player) -> Self {
        Self { player_a, player_b, turn_count: 0 }
    }

    /// Start the main game loop. At this point in the code,
    /// we should expect that the creation of the game and the player
    /// has been done, and we only care about managing game state
    /// between successive turns from each player.
    pub fn start_loop() -> Result<(), GameError> {

        'gameLoop: loop {
            break 'gameLoop;   
        }

        Ok(())
    }
}