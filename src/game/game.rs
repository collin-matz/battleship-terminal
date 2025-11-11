/// Module for housing game logic and management.
use std::vec;
use thiserror::Error;
use crate::game::components::{board, player, ship};
use crate::game::layouts;

use crossterm::{
    cursor,
    terminal,
    execute
};


pub enum GameEndReason {
    PlayerAWon,
    PlayerBWon,
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

    pub fn get_player_a(&self) -> &player::Player {
        &self.player_a
    }

    pub fn get_player_b(&self) -> &player::Player {
        &self.player_b
    }

    /// Start the main game loop. At this point in the code,
    /// we should expect that the creation of the game and the player
    /// has been done, and we only care about managing game state
    /// between successive turns from each player.
    pub fn start_loop(&mut self) -> std::io::Result<GameEndReason> {

        // for the entire game loop, we'll be in an alternate terminal, so we do that once here
        terminal::enable_raw_mode()?;
        let mut out = std::io::stdout();
        execute!(out, terminal::EnterAlternateScreen, cursor::Hide, terminal::Clear(terminal::ClearType::All))?;

        let mut player_a_cursor_pos: (usize, usize) = (0, 0);
        self.turn_count = 1;

        let main_loop_exit_option: std::io::Result<GameEndReason> = 'gameLoop: loop {
            // 2. render the current board states for both players
            let exit_option: std::io::Result<Option<(usize, usize)>> = layouts::game::main_loop::show_once(
                &mut out,
                self.turn_count,
                &mut self.player_a, 
                &mut self.player_b, 
                &mut player_a_cursor_pos
            );

            match exit_option {
                Ok(selected_indices) => {
                    if let Some((row, col)) = selected_indices {
                        // apply the guessed location to player B's board
                        self.player_b.guess(row, col);

                        // play the computer's turn
                        self.player_a.auto_guess();

                        // check for win condition
                        if self.player_b.all_ships_sunk() {
                            break 'gameLoop Ok(GameEndReason::PlayerAWon);
                        } else if self.player_a.all_ships_sunk() {
                            break 'gameLoop Ok(GameEndReason::PlayerBWon);
                        }

                        // increment turn count if no win
                        self.turn_count += 1;
                    }
                }, // continue the game loop
                Err(e) => break 'gameLoop Err(e)  // exit the game loop with the error
            };
            
        };

        // exit the alternate screen on game end
        execute!(out, cursor::Show, terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        main_loop_exit_option
    }
}