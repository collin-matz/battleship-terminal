/// Module for housing game logic and management.
use std::vec;
use thiserror::Error;
use crate::game::components::{board, player, ship};


#[derive(Debug, Error)]
pub enum GameError {
    #[error("Game unexpectedly crashed")]
    UnknownError
}

pub struct Game<'a> {
    player_a: player::Player<'a>,
    player_b: player::Player<'a>,
    // we make turn count a 'usize' instead of something like 'u16' or 'u32'
    // because the turn_count is dictated by the board size, and since
    // board size is type 'usize', it would follow that turn_count should be also
    turn_count: usize,
}

impl<'a> Game<'a> {
    /// Create a new game instance with the two players and a turn count of 0.
    pub fn new(player_a: player::Player<'a>, player_b: player::Player<'a>) -> Self {
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