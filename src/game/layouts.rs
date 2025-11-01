/// This module contains all layout modules, as well as the
/// layout trait for defining what each module should
/// conform to.
use std::{io::Write, fmt};
use colored::Colorize;
use crossterm::{
    event,
    cursor,
    terminal,
    style,
    execute, queue
};

/// Trait for defining each terminal layout module.
pub trait TerminalLayout<T> {
    fn show() -> std::io::Result<T>;
}

/// This module contains logic for managing states of menus.
pub mod menus {
    use super::*;

    /// Static reference to the title, stored in "title.txt"
    const TITLE: &str = include_str!("title.txt");

    /// Module for displaying the main menu.
    pub mod main_menu {
        use super::*;

        /// An enum defining all possible menu options.
        #[derive(Clone)]
        pub enum MainMenuOptions {
            NewGame,
            Statistics,
            Quit
        }

        impl fmt::Display for MainMenuOptions {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    MainMenuOptions::NewGame => write!(f, "New Game"),
                    MainMenuOptions::Statistics => write!(f, "Statistics"),
                    MainMenuOptions::Quit => write!(f, "Quit")
                }
            }
        }

        impl MainMenuOptions {
            /// A static array containing all possible menu options to iterate over.
            const ALL: [MainMenuOptions; 3] = [
                MainMenuOptions::NewGame,
                MainMenuOptions::Statistics,
                MainMenuOptions::Quit
            ];

            /// Generate a consuming iterator over the menu options
            pub fn iter() -> impl Iterator<Item = MainMenuOptions> {
                Self::ALL.into_iter()
            }
        }

        pub struct MainMenu;

        impl TerminalLayout<MainMenuOptions> for MainMenu {
            /// Display the main menu in the terminal.
            fn show() -> std::io::Result<MainMenuOptions> {
                // color the title string for the menu
                let title: colored::ColoredString = format!("{}\n\n", TITLE).red();

                // enter an alternate screen for the main menu
                terminal::enable_raw_mode()?;
                let mut out = std::io::stdout();
                execute!(out, terminal::EnterAlternateScreen, cursor::Hide, terminal::Clear(terminal::ClearType::All))?;

                // begin rendering loop. at the end of this loop, we get returned an option that
                // the user selected that we can use to move to another screen in the layout
                let mut selected: usize = 0;
                let selection: MainMenuOptions = 'render: loop {
                    // clear terminal and print the title and movement commands
                    queue!(out, cursor::MoveTo(0, 0), terminal::Clear(terminal::ClearType::All))?;
                    queue!(out, style::Print(&title))?;
                    queue!(out, style::Print("Use ↑/↓ to move, Esc to exit\n\n"))?;

                    // enumerate over the menu options and display each
                    for (i, option) in MainMenuOptions::iter().enumerate() {
                        // if the current selected item is the one we're iterating over,
                        // apply a reverse highlight to that element to indicate to the user
                        // that they have selected this
                        if i == selected {
                            queue!(out, style::SetAttribute(style::Attribute::Reverse))?;
                        }

                        // print a right facing arrow on the selected option. print each options's text
                        queue!(out, style::Print(format!(" {} {}\n", if i == selected { ">" } else { " " }, option)))?;

                        // if we just highlighted the selected text, we need to undo this highlight for
                        // the text below, so we add a no-reverse highlight after
                        if i == selected {
                            queue!(out, style::SetAttribute(style::Attribute::NoReverse))?;
                        }
                    }

                    // write all output to the screen
                    out.flush()?;

                    // poll for the last event that occurred
                    if let event::Event::Key(key) = event::read()? {
                        if key.kind == event::KeyEventKind::Press {
                            match key.code {
                                // rem_euclid always returns a positive int, so it handles negatives natively.
                                // with this logic, pressing up or down cycles back to the other end of the menu
                                // while navigating.
                                event::KeyCode::Up => selected = (selected - 1).rem_euclid(MainMenuOptions::ALL.len()),
                                event::KeyCode::Down => selected = (selected + 1) % MainMenuOptions::ALL.len(),

                                // get the menu option selected by the user and return it
                                event::KeyCode::Enter => break 'render MainMenuOptions::ALL[selected].clone(),

                                // quit game if the user hits Esc
                                event::KeyCode::Esc => break 'render MainMenuOptions::Quit,
                                _ => {}
                            }
                        }
                    }          
                };

                // leave the main menu screen.
                execute!(out, cursor::Show, terminal::LeaveAlternateScreen)?;
                terminal::disable_raw_mode()?;

                // return an Ok with the selected menu option
                Ok(selection)
            }
        }
    }

    /// Module for displaying the new game menu.
    pub mod new_game_menu {
        use super::*;

        /// An enum defining all possible menu options.
        #[derive(Clone)]
        pub enum NewGameMenuOptions {
            PlayComputer,
            JoinGame,
            HostGame,
            Back
        }

        impl fmt::Display for NewGameMenuOptions {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    NewGameMenuOptions::PlayComputer => write!(f, "Player against Computer"),
                    NewGameMenuOptions::JoinGame => write!(f, "Join Game"),
                    NewGameMenuOptions::HostGame => write!(f, "Host Game"),
                    NewGameMenuOptions::Back => write!(f, "Back")
                }
            }
        }

        impl NewGameMenuOptions {
            /// A static array containing all possible menu options to iterate over.
            const ALL: [NewGameMenuOptions; 4] = [
                NewGameMenuOptions::PlayComputer,
                NewGameMenuOptions::JoinGame,
                NewGameMenuOptions::HostGame,
                NewGameMenuOptions::Back
            ];

            /// Generate a consuming iterator over the menu options
            pub fn iter() -> impl Iterator<Item = NewGameMenuOptions> {
                Self::ALL.into_iter()
            }
        }

        pub struct NewGameMenu;

        impl TerminalLayout<NewGameMenuOptions> for NewGameMenu {
            /// Display the new game menu in the terminal.
            fn show() -> std::io::Result<NewGameMenuOptions> {
                // color the title string for the menu
                let title: colored::ColoredString = format!("{}\n\n", TITLE).red();

                // enter an alternate screen for menu
                terminal::enable_raw_mode()?;
                let mut out = std::io::stdout();
                execute!(out, terminal::EnterAlternateScreen, cursor::Hide, terminal::Clear(terminal::ClearType::All))?;

                // begin rendering loop. at the end of this loop, we get returned an option that
                // the user selected that we can use to move to another screen in the layout
                let mut selected: usize = 0;
                let selection: NewGameMenuOptions = 'render: loop {
                    // clear terminal and print the title and movement commands
                    queue!(out, cursor::MoveTo(0, 0), terminal::Clear(terminal::ClearType::All))?;
                    queue!(out, style::Print(&title))?;
                    queue!(out, style::Print("Use ↑/↓ to move, Esc to go back\n\n"))?;

                    // enumerate over the menu options and display each
                    for (i, option) in NewGameMenuOptions::iter().enumerate() {
                        // if the current selected item is the one we're iterating over,
                        // apply a reverse highlight to that element to indicate to the user
                        // that they have selected this
                        if i == selected {
                            queue!(out, style::SetAttribute(style::Attribute::Reverse))?;
                        }

                        // print a right facing arrow on the selected option. print each options's text
                        queue!(out, style::Print(format!(" {} {}\n", if i == selected { ">" } else { " " }, option)))?;

                        // if we just highlighted the selected text, we need to undo this highlight for
                        // the text below, so we add a no-reverse highlight after
                        if i == selected {
                            queue!(out, style::SetAttribute(style::Attribute::NoReverse))?;
                        }
                    }

                    // write all output to the screen
                    out.flush()?;

                    // poll for the last event that occurred
                    if let event::Event::Key(key) = event::read()? {
                        if key.kind == event::KeyEventKind::Press {
                            match key.code {
                                // rem_euclid always returns a positive int, so it handles negatives natively.
                                // with this logic, pressing up or down cycles back to the other end of the menu
                                // while navigating.
                                event::KeyCode::Up => selected = (selected - 1).rem_euclid(NewGameMenuOptions::ALL.len()),
                                event::KeyCode::Down => selected = (selected + 1) % NewGameMenuOptions::ALL.len(),

                                // get the menu option selected by the user and return it
                                event::KeyCode::Enter => break 'render NewGameMenuOptions::ALL[selected].clone(),

                                // quit game if the user hits Esc
                                event::KeyCode::Esc => break 'render NewGameMenuOptions::Back,
                                _ => {}
                            }
                        }
                    }          
                };

                // leave the main menu screen.
                execute!(out, cursor::Show, terminal::LeaveAlternateScreen)?;
                terminal::disable_raw_mode()?;

                // return an Ok with the selected menu option
                Ok(selection)
            }
        }
    }
    
}

pub mod game {
    use super::*;
    use crate::game::components::{board, player, ship};

    pub mod board_setup {
        use std::vec;
        use crate::game::components::board::Board;

        use super::*;

        /// An enum defining all possible ship orientations.
        enum ShipOrientation {
            Left,
            Up,
            Right,
            Down
        }

        impl ShipOrientation {
            /// Given the current ship orientation, return the next orientation in clockwise order.
            fn next(&self) -> ShipOrientation {
                match self {
                    ShipOrientation::Left => ShipOrientation::Up,
                    ShipOrientation::Up => ShipOrientation::Right,
                    ShipOrientation::Right => ShipOrientation::Down,
                    ShipOrientation::Down => ShipOrientation::Left,
                }
            }
        }

        /// Try to place a ship on the player's board. If successful, returns true. Otherwise, returns false.
        fn get_ship_placement_cell_states(
            ship_type: &ship::ShipType, 
            orientation: &ShipOrientation,
            selected_cell: &(usize, usize)
        ) -> (vec::Vec<(usize, usize)>, board::CellState) {
            // clone the selected cell so we can modify it internally
            let mut current = selected_cell.clone();

            // initialize vector for indices and cell state that will be rendered
            let mut indices: vec::Vec<(usize, usize)> = vec![];
            let mut state: board::CellState = board::CellState::Highlighted;

            // loop over the size of this ship and match the orientation to determine
            // which cells to try to highlight
            for _ in 0..ship_type.size() {
                indices.push(current);
                match orientation {
                    ShipOrientation::Left => {
                        if current.1 > 0 { current.1 -= 1; } else { 
                            current.1 = board::COLS - 1;
                            state = board::CellState::InvalidPlacement;
                            break;
                        };
                    },
                    ShipOrientation::Right => {
                        if current.1 < board::COLS { current.1 += 1; } else { 
                            current.1 = 0;
                            state = board::CellState::InvalidPlacement;
                            break;
                        }
                    },
                    ShipOrientation::Up => {
                        if current.0 > 0 { current.0 -= 1; } else { 
                            current.0 = board::ROWS - 1;
                            state = board::CellState::InvalidPlacement;
                            break;
                        }
                    },
                    ShipOrientation::Down => {
                        if current.0 < board::ROWS  { current.0 += 1; } else { 
                            current.0 = 0;
                            state = board::CellState::InvalidPlacement;
                            break;
                        }
                    },
                }
            };
            (indices, state)
        }
        
        /// Display the board setup in the terminal.
        pub fn show(player: &mut player::Player) -> std::io::Result<()> {
            // enter an alternate screen
            terminal::enable_raw_mode()?;
            let mut out = std::io::stdout();
            execute!(out, terminal::EnterAlternateScreen, cursor::Hide, terminal::Clear(terminal::ClearType::All))?;

            // set the necessary values for tracking the ship placement state
            let mut selected: (usize, usize) = (0, 0);
            let mut ship_selection: usize = 0;
            let mut selected_ship_type = ship::ShipType::ALL[ship_selection];
            let mut cell_indices: vec::Vec<(usize, usize)>;
            let mut ship_orientation: ShipOrientation = ShipOrientation::Left;
            let mut cell_state_type: board::CellState;

            // begin rendering loop. at the end of this loop, we get returned an option that
            // the user has completed setting up and that the game is ready to progress to
            // the next stage
            'render: loop {
                // clear terminal and print the title and movement commands
                queue!(out, cursor::MoveTo(0, 0), terminal::Clear(terminal::ClearType::All))?;
                queue!(out, style::Print("Use ←/↑/→/↓ to move, Esc to quit the game\n\n"))?;
                
                for (i, ship) in ship::ShipType::iter().enumerate() {
                    if i == ship_selection {
                        queue!(out, style::SetAttribute(style::Attribute::Reverse))?;
                    }

                    queue!(out, cursor::MoveTo(i as u16 * 15, 2), style::Print(ship))?;

                    if i == ship_selection {
                        queue!(out, style::SetAttribute(style::Attribute::NoReverse))?;
                    }
                }

                // find the ship that corresponds to the currently selected index
                selected_ship_type = ship::ShipType::ALL[ship_selection];
                (cell_indices, cell_state_type) = get_ship_placement_cell_states(&selected_ship_type, &ship_orientation, &selected);

                // print each cell in the board
                for r in 0..board::ROWS {
                    for c in 0..board::COLS {

                        // undo highlight to the current cell 
                        player.get_cell_mut(r, c).undo();
                        
                        if cell_indices.contains(&(r,c)) {
                            match cell_state_type {
                                board::CellState::Highlighted => player.get_cell_mut(r, c).highlight(),
                                board::CellState::InvalidPlacement => player.get_cell_mut(r, c).invalidate(),
                                _ => {}  // if not one of these two states, do nothing
                            }
                        }

                        queue!(out, cursor::MoveTo((c as u16) * 3 , (r as u16)  + 4), style::Print(player.get_cell(r, c)))?;
                    }
                }

                // write all output to the screen
                out.flush()?;

                // poll for the last event that occurred
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Press {
                        match key.code {
                            event::KeyCode::Up => selected.0 = if selected.0 == 0 { board::ROWS - 1 } else { selected.0 - 1 },
                            event::KeyCode::Down => selected.0 = (selected.0 + 1) % board::ROWS,
                            event::KeyCode::Left => selected.1 = if selected.1 == 0 { board::COLS - 1 } else { selected.1 - 1 },
                            event::KeyCode::Right => selected.1 = (selected.1 + 1) % board::COLS,
                            // allow for caps lock
                            event::KeyCode::Char('q') | event::KeyCode::Char('Q') => ship_orientation = ship_orientation.next(),

                            // if tab, swap through the selected ships
                            event::KeyCode::Tab => ship_selection = (ship_selection + 1) % ship::ShipType::ALL.len(),

                            // try to confirm the ship selection if valid. otherwise, do nothing
                            event::KeyCode::Enter => {
                                if true {
                                    player.add_ship(cell_indices, selected_ship_type);
                                }
                            },

                            // break render loop if user hits esc
                            event::KeyCode::Esc => break 'render,
                            _ => {}
                        }
                    }
                }          
            };

            // leave the main menu screen.
            execute!(out, cursor::Show, terminal::LeaveAlternateScreen)?;
            terminal::disable_raw_mode()?;

            // return an Ok with the selected menu option
            Ok(())
        }
    }
}
