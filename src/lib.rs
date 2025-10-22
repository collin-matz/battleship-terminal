pub mod components;

/// This module contains logic for managing game state.
pub mod game {
    // import the game components 
    use crate::components::*;

    use std::io::Write;
    use colored::Colorize;
    use crossterm::{
        event,
        cursor,
        terminal,
        style,
        execute, queue
    };
    use rand::random;

    /// An enum to define the different stages the game can be in.
    /// 
    /// This is used in an effort to reuse logic in the main loop. We can
    /// conditionally render what we need based on the assigned stage,
    /// rather than have multiple branches with copies of the same code.
    enum Stage {
        Setup,  // players setup their ships
        Play  // players play their turns
    }

    /// A struct for encapsulating game state and logic.
    pub struct Game {
        // we limit the total number of moves to be <= the total cell count.
        turn: usize,  
        max_allowed_turns: usize,
        a: player::Player,
        b: player::Player,
        stage: Stage
    }

    impl Game {
        /// Create a new game instance and begin the game logic.
        pub fn new(a_name: &'static str, b_name: &'static str) -> Self {
            Self {
                turn: 0,
                max_allowed_turns: board::ROWS * board::COLS,
                a: player::Player::new(a_name),
                b: player::Player::new(b_name),
                stage: Stage::Setup
            }
        }  

        /// Start the game.
        pub fn start(&mut self) -> std::io::Result<()> {
            
            // begin the main menu loop
            let selection: menu::MainMenuOptions = menu::show_main_menu()?;

            // if the user selected quit, return early here
            if let menu::MainMenuOptions::Quit = selection {
                println!("\nThanks for playing! :)");
                return Ok(())
            }

            // enter the main loop
            self.main_loop()?;

            Ok(())
        }

        /// Start the main game loop.
        fn main_loop(&mut self) -> std::io::Result<()> {

            // send the terminal into raw mode and enter a new screen to facilitate the gameplay
            terminal::enable_raw_mode()?;
            let mut out = std::io::stdout();
            execute!(out, terminal::EnterAlternateScreen, cursor::Hide, terminal::Clear(terminal::ClearType::All))?;

            let current = &self.a;  // assign the current player. we'll update this as the turns go on.
            let mut selected_ship: u8 = 0;  // this is only used for setup; it's how we know what ship top toggle with

            // begin the main game loop
            'game: loop {

                // populate the titles at the top of the screen
                queue!(out, cursor::MoveTo(0, 0), terminal::Clear(terminal::ClearType::All))?;
                queue!(out, style::Print(self.get_instructions_string()))?;
                self.get_extras_string(current, selected_ship);

                // write to the terminal
                out.flush()?;

                // poll for the last event that occurred
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Press {
                        match key.code {
                            // if the current stage is Setup, then we want to cycle through the ship options
                            event::KeyCode::Tab => {
                                if let Stage::Setup = self.stage {
                                    selected_ship = (selected_ship + 1) % (current.get_ships().len() as u8);
                                }
                            }
                            event::KeyCode::Esc => break 'game,
                            _ => {}
                        }
                    }
                }        
            };

            // exit the alternate screen and disable raw mode for the terminal
            execute!(out, cursor::Show, terminal::LeaveAlternateScreen)?;
            terminal::disable_raw_mode()?;

            Ok(())
        }

        /// Return the instructions string for this stage.
        fn get_instructions_string(&self) -> String {
            match self.stage {
                Stage::Setup => format!(
                    "Use {} to move across the board, {} to cycle through ship types, {} to toggle the board cell, {} to exit\n",
                    "←/↑/→/↓".bold().yellow(), "Tab".bold().yellow(), "Space".bold().yellow(), "Esc".bold().yellow()
                ),
                Stage::Play => format!(
                    "Use {}  to move across the board, {} to toggle the board cell, {} to exit\n",
                    "←/↑/→/↓".bold().yellow(), "Space".bold().yellow(), "Esc".bold().yellow()
                )
            }
        }

        /// Return the extras string for this stage.
        fn get_extras_string(&self, current: &player::Player, mut selected_ship: u8) -> std::io::Result<()> {
            let mut out = std::io::stdout();

            match self.stage {
                Stage::Setup => {
                    for (i, ship) in current.get_ships().iter().enumerate() {
                        if (i as u8) == selected_ship {
                            // highlight the selected ship
                            queue!(out, style::SetAttribute(style::Attribute::Reverse))?;
                        }

                        queue!(out, style::Print(format!("{}\t\t", ship.get_ship_type())))?;

                        if (i as u8) == selected_ship {
                            // highlight the selected ship
                            queue!(out, style::SetAttribute(style::Attribute::NoReverse))?;
                        }
                    }
                    Ok(())
                },

                Stage::Play => {

                    Ok(())
                }
            }
        }
    }
}