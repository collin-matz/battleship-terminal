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

        use super::*;

        /// An Enum defining the possible states that can be returned from the render loop.
        pub enum ShipSetupOption {
            Continue,
            Quit
        }

        /// Try to place a ship on the player's board. If successful, returns true. Otherwise, returns false.
        fn get_ship_placement_cell_states(
            ship_type: &ship::ShipType, 
            orientation: &ship::ShipOrientation,
            selected_cell: &(usize, usize),
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
                    ship::ShipOrientation::Left => {
                        if current.1 > 0 { current.1 -= 1; } else { 
                            current.1 = board::COLS - 1;
                            state = board::CellState::InvalidPlacement;
                            break;
                        };
                    },
                    ship::ShipOrientation::Right => {
                        if current.1 < board::COLS { current.1 += 1; } else { 
                            current.1 = 0;
                            state = board::CellState::InvalidPlacement;
                            break;
                        }
                    },
                    ship::ShipOrientation::Up => {
                        if current.0 > 0 { current.0 -= 1; } else { 
                            current.0 = board::ROWS - 1;
                            state = board::CellState::InvalidPlacement;
                            break;
                        }
                    },
                    ship::ShipOrientation::Down => {
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
        pub fn show(player: &mut player::Player) -> std::io::Result<ShipSetupOption> {
            // enter an alternate screen
            terminal::enable_raw_mode()?;
            let mut out = std::io::stdout();
            execute!(out, terminal::EnterAlternateScreen, cursor::Hide, terminal::Clear(terminal::ClearType::All))?;

            // set the necessary values for tracking the ship placement state
            let mut selected: (usize, usize) = (0, 0);
            let mut ship_selection: usize = 0;
            let mut ship_has_been_placed: vec::Vec<bool> = vec![false; ship::ShipType::ALL.len()];
            let mut selected_ship_type: ship::ShipType = ship::ShipType::ALL[ship_selection];
            let mut cell_indices: vec::Vec<(usize, usize)>;
            let mut ship_orientation: ship::ShipOrientation = ship::ShipOrientation::Left;
            let mut cell_state_type: board::CellState;

            // begin rendering loop. at the end of this loop, we get returned an option that
            // the user has completed setting up and that the game is ready to progress to
            // the next stage
            let selected_ship_setup_option: ShipSetupOption = 'render: loop {
                // clear terminal and print the title and movement commands
                queue!(out, cursor::MoveTo(0, 0), terminal::Clear(terminal::ClearType::All))?;
                queue!(out, style::Print("Use ←/↑/→/↓ to move, R to rotate the ship's orientation, Esc to quit the game\n\n"))?;
                
                for (i, ship) in ship::ShipType::iter().enumerate() {
                    // highlight the currently selected ship
                    if i == ship_selection {
                        queue!(out, style::SetAttribute(style::Attribute::Reverse))?;
                    }
                    // if the ship has been placed, gray it out
                    if ship_has_been_placed[i] {
                        queue!(out, style::SetForegroundColor(style::Color::DarkGrey))?;
                    }
                    
                    queue!(out, cursor::MoveTo(i as u16 * 15, 2), style::Print(ship))?;

                    // reset styles after printing
                    if ship_has_been_placed[i] {
                        queue!(out, style::SetForegroundColor(style::Color::Reset))?;
                    }
                    if i == ship_selection {
                        queue!(out, style::SetAttribute(style::Attribute::NoReverse))?;
                    }
                }

                if (ship_has_been_placed.iter().all(|x| x == &true)) {
                    queue!(out, cursor::MoveTo(ship::ShipType::ALL.len() as u16 * 15, 2), style::Print("Press Enter to Continue"))?;
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
                            event::KeyCode::Char('r') | event::KeyCode::Char('R') => ship_orientation = ship_orientation.next(),

                            // if tab, swap through the selected ships
                            event::KeyCode::Tab => ship_selection = (ship_selection + 1) % ship::ShipType::ALL.len(),

                            // try to confirm the ship selection if valid. otherwise, do nothing
                            event::KeyCode::Enter => {
                                // if enter is pressed before all ships are placed, try to place the selected ship
                                if (!ship_has_been_placed[ship_selection]) && (cell_state_type != board::CellState::InvalidPlacement) {
                                    player.add_ship(cell_indices, selected_ship_type);
                                    ship_has_been_placed[ship_selection] = true;
                                }
                                // else, if all ships have been placed, exit the setup loop
                                else if ship_has_been_placed.iter().all(|x| x == &true) {
                                    // before continuing, undo the cell highlights
                                    for r in 0..board::ROWS {
                                        for c in 0..board::COLS {
                                            // undo highlight to the current cell 
                                            player.get_cell_mut(r, c).undo();
                                        }
                                    }
                                    break 'render ShipSetupOption::Continue;
                                }
                            },

                            // break render loop if user hits esc
                            event::KeyCode::Esc => break 'render ShipSetupOption::Quit,
                            _ => {}
                        }
                    }
                }          
            };

            // leave the main menu screen.
            execute!(out, cursor::Show, terminal::LeaveAlternateScreen)?;
            terminal::disable_raw_mode()?;

            // return an Ok with the selected menu option
            Ok(selected_ship_setup_option)
        }
    }

    pub mod main_loop {

        use super::*;

        // constant for offsetting opponent's board rendering
        const OPPONENT_BOARD_OFFSET: u16 = 60;

        pub fn show_once(
            out: &mut std::io::Stdout, 
            turn_count: usize,
            player: &mut player::Player, 
            opponent: &mut player::Player,
            player_a_cursor_position: &mut(usize, usize)
        ) -> std::io::Result<Option<(usize, usize)>> {
            // clear terminal and print the title and movement commands
            queue!(out, cursor::MoveTo(0, 0), terminal::Clear(terminal::ClearType::All))?;
            queue!(out, style::Print("Use ←/↑/→/↓ to move, Enter to guess a location on the opponent's board, Esc to quit the game\n\n"))?;
            queue!(out, style::Print(format!("TURN: {}\n\n", turn_count)))?;

            // print each cell in the board
            for r in 0..board::ROWS {
                for c in 0..board::COLS {

                    // undo highlight to the current cell 
                    opponent.get_cell_mut(r, c).undo();
                    
                    if &(r, c) == player_a_cursor_position {
                        opponent.get_cell_mut(r, c).highlight();
                    }

                    // print both the player's and opponent's boards
                    queue!(out, cursor::MoveTo((c as u16) * 3 , (r as u16)  + 4), style::Print(player.get_cell(r, c)))?;
                    queue!(out, cursor::MoveTo((c as u16) * 3 + OPPONENT_BOARD_OFFSET , (r as u16)  + 4), style::Print(opponent.get_hidden_cell(r, c)))?;
                }
            }

            // write all output to the screen
            out.flush()?;

            // poll for the last event that occurred
            if let event::Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Up => player_a_cursor_position.0 = if player_a_cursor_position.0 == 0 { board::ROWS - 1 } else { player_a_cursor_position.0 - 1 },
                        event::KeyCode::Down => player_a_cursor_position.0 = if player_a_cursor_position.0 == board::ROWS - 1 { 0 } else { player_a_cursor_position.0 + 1 },
                        event::KeyCode::Left => player_a_cursor_position.1 = if player_a_cursor_position.1 == 0 { board::COLS - 1 } else { player_a_cursor_position.1 - 1 },
                        event::KeyCode::Right => player_a_cursor_position.1 = if player_a_cursor_position.1 == board::COLS - 1 { 0 } else { player_a_cursor_position.1 + 1 },
                        event::KeyCode::Enter => {
                            return Ok(Some(*player_a_cursor_position));
                        },
                        event::KeyCode::Esc => {
                            return Err(std::io::Error::new(std::io::ErrorKind::Other, "User exited game"));
                        },
                        _ => {}
                    }
                }
            } 

            Ok(None)
        }
    }


    pub mod win_screen {

        use super::*;

        // constant for offsetting opponent's board rendering
        const OPPONENT_BOARD_OFFSET: u16 = 60;

        pub fn show(
            player: &player::Player, 
            opponent: &player::Player,
            winner: &str
        ) -> std::io::Result<()> {

            // enter an alternate screen for the win screen
            terminal::enable_raw_mode()?;
            let mut out = std::io::stdout();
            execute!(out, terminal::EnterAlternateScreen, cursor::Hide, terminal::Clear(terminal::ClearType::All))?;
            
            loop {
                // clear terminal and print the title and movement commands
                queue!(out, cursor::MoveTo(0, 0), terminal::Clear(terminal::ClearType::All))?;
                queue!(out, style::Print("Press Esc to quit the game\n\n"))?;
                queue!(out, style::Print(format!("Winner: {}!\n\n", winner)))?;

                // print each cell in the board
                for r in 0..board::ROWS {
                    for c in 0..board::COLS {
                        // print both the player's and opponent's boards
                        queue!(out, cursor::MoveTo((c as u16) * 3 , (r as u16)  + 4), style::Print(player.get_cell(r, c)))?;
                        queue!(out, cursor::MoveTo((c as u16) * 3 + OPPONENT_BOARD_OFFSET , (r as u16)  + 4), style::Print(opponent.get_cell(r, c)))?;
                    }
                }

                // write all output to the screen
                out.flush()?;

                // poll for the last event that occurred
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Press {
                        match key.code {
                            event::KeyCode::Esc => {
                                break;
                            },
                            _ => {}
                        }
                    }
                } 
            }

            // leave the win screen.
            execute!(out, cursor::Show, terminal::LeaveAlternateScreen)?;
            terminal::disable_raw_mode()?;

            Ok(())
        }
    }
}
