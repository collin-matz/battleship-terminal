/// This module contains logic for managing game state.
pub mod game {
    use std::io::Write;

    use crate::board;
    use crate::board::Ship;
    use crate::menu;
    use colored::Colorize;
    use crossterm::{
        event,
        cursor,
        terminal,
        style,
        execute, queue
    };

    /// An enum for managing the players. This way,
    /// we can reuse the same logic for each and simply check
    /// which player to update.
    enum Player { A, B }

    /// A struct for encapsulating game state and logic.
    pub struct Game {
        // We limit the total number of moves to be <= the total cell count.
        // As such, we need to make both variables type usize to match the _ROWS
        // and _CELLS counts.
        turn: usize,  
        max_allowed_turns: usize,
        current_player: Player,
        player_1_board: board::Board,
        player_2_board: board::Board,
        player_1_cursor: (usize, usize),
        player_1_ships: [board::Ship; 5],
        player_2_ships: [board::Ship; 5]
    }

    impl Game {
        /// Create a new game instance and begin the game logic.
        pub fn new() -> Self {
            Self {
                turn: 0,
                max_allowed_turns: board::ROWS * board::COLS,
                current_player: Player::A,
                player_1_board: board::Board::new_empty(),
                player_2_board: board::Board::new_empty(),
                player_1_cursor: (0, 0),
                player_1_ships: Ship::get_all_ships(),
                player_2_ships: Ship::get_all_ships()
            }
        }  

        pub fn start_loop(&mut self) -> std::io::Result<()> {
            // begin the main menu loop. once the selections are complete,
            // break this loop and enter the game loop
            let selection: menu::MenuOptions = 'menu: loop {
                let selected = menu::MainMenu::show();
                match selected {
                    Ok(option) => match option {
                        menu::MenuOptions::NewGame(_) => break 'menu menu::MenuOptions::NewGame(""),
                        menu::MenuOptions::Statistics(_) => println!("Going to stats..."),
                        menu::MenuOptions::Quit(_) => break 'menu menu::MenuOptions::Quit("")
                    }
                    Err(e) => {
                        println!("{}", e);
                        break 'menu menu::MenuOptions::Quit("");
                    }
                }
            };

            // if the user selected quit, return early here
            if let menu::MenuOptions::Quit("") = selection {
                println!("Thanks for playing!");
                return Ok(())
            }

            // send the terminal into raw mode and enter a new screen to facilitate the gameplay
            terminal::enable_raw_mode()?;
            let mut out = std::io::stdout();
            execute!(out, terminal::EnterAlternateScreen, cursor::Hide, terminal::Clear(terminal::ClearType::All))?;
            self.setup()?;

            // begin the main game loop
            'game: loop {
                break 'game;
            };

            terminal::disable_raw_mode()?;
            execute!(out, cursor::Show, terminal::LeaveAlternateScreen)?;

            Ok(())
        }

        /// Update the player_1 cursor position. If the update would take the cursor
        /// out of bounds, do nothing.
        fn move_player_1_cursor(&mut self, r_delta: i16, c_delta: i16) {
            let mut new_cursor_r = ((self.player_1_cursor.0 as i16) + r_delta) as usize;
            let mut new_cursor_c = ((self.player_1_cursor.1 as i16) + c_delta) as usize;

            if (new_cursor_r >= 0 && new_cursor_r < board::ROWS) {
                new_cursor_r = new_cursor_r as usize;
                self.player_1_cursor.0 = new_cursor_r
            }
            if (new_cursor_c >= 0 && new_cursor_c < board::COLS) {
                new_cursor_c = new_cursor_c as usize;
                self.player_1_cursor.1 = new_cursor_c;
            }
        }

        /// Allow both players to setup their boards.
        fn setup(&mut self) -> std::io::Result<()> {
            let mut out = std::io::stdout();
            let mut selected_ship: usize = 0;

            'setup: loop {
                queue!(out, cursor::MoveTo(0, 0), terminal::Clear(terminal::ClearType::All))?;
                queue!(out, style::Print("Use the arrow keys to move, Tab to cycle through ship types, Spacebar to toggle, Esc to exit\n\n"))?;
                queue!(out, style::Print("Remaining ships to place: "))?;

                // print all ships and highlight the current selected ship
                for (i, ship) in self.player_1_ships.iter().enumerate() {
                    if i == selected_ship {
                        queue!(out, style::SetAttribute(style::Attribute::Reverse))?;
                    }
                    queue!(out, style::Print(format!("{}\t", ship.ship_type)))?;
                    queue!(out, style::SetAttribute(style::Attribute::NoReverse))?;
                }

                for r in 0..board::ROWS {
                    queue!(out, cursor::MoveTo(0, 0), style::Print("\n"))?;

                    for c in 0..board::COLS {
                        // highlight the cursor cell
                        if (r, c) == self.player_1_cursor {
                            self.player_1_board.update(r, c, board::Cell::Highlighted);
                        }
                        else {
                            self.player_1_board.update(r, c, board::Cell::Empty);
                        }
                        queue!(out, cursor::MoveTo(2 + c as u16 * 4, 5 + r as u16), style::Print(format!(" {} ", self.player_1_board.get(r,c))))?;
                    }

                    // print player_2's board
                    for c in 0..board::COLS {
                        queue!(out, cursor::MoveTo(60 + c as u16 * 4, 5 + r as u16), style::Print(format!(" {} ", self.player_2_board.get(r,c))))?;
                    }
                }

                queue!(out, cursor::MoveTo(14 as u16, 15 as u16), style::Print("Your Board".green()))?;
                queue!(out, cursor::MoveTo(71 as u16, 15 as u16), style::Print("Opponent's Board".red()))?;
                

                // flush output to the terminal
                out.flush()?;

                if let Ok(event::Event::Key(key)) = event::read() {
                    // only react on press to avoid repeats/releases
                    if key.kind == event::KeyEventKind::Press {
                        match key.code {
                            event::KeyCode::Up => self.move_player_1_cursor(-1, 0),
                            event::KeyCode::Down => self.move_player_1_cursor(1, 0),
                            event::KeyCode::Left => self.move_player_1_cursor(0, -1),
                            event::KeyCode::Right => self.move_player_1_cursor(0, 1),
                            event::KeyCode::Tab => selected_ship = (selected_ship + 1) % board::NUM_SHIPS,
                            event::KeyCode::Esc => break 'setup,
                            _ => {}
                        }
                    }
                }       
            };

            Ok(())
        }                            
    }
}

/// This module contains logic for managing menu state.
mod menu {
    use std::io::Write;
    use colored::Colorize;
    use crossterm::{
        event,
        cursor,
        terminal,
        style,
        execute, queue
    };

    pub enum MenuOptions {
        NewGame(&'static str),
        Statistics(&'static str),
        Quit(&'static str)
    }

    impl MenuOptions {
        const ALL: [MenuOptions; 3] = [
            MenuOptions::NewGame("New Game"),
            MenuOptions::Statistics("Statistics"),
            MenuOptions::Quit("Quit")
        ];

        pub fn iter() -> impl Iterator<Item = MenuOptions> {
            Self::ALL.into_iter()
        }
    }

    const TITLE: &str = include_str!("title.txt");

    /// A structure for encapsulating main menu data and logic.
    pub struct MainMenu;

    impl MainMenu {
        /// Create a new main menu and control rendering and option selection.
        pub fn show() -> std::io::Result<MenuOptions>{

            // read in and color the title
            let title: colored::ColoredString = format!("{}\n\n", TITLE).red();

            // enter an alternate screen for the main menu
            terminal::enable_raw_mode()?;
            let mut out = std::io::stdout();
            execute!(out, terminal::EnterAlternateScreen, cursor::Hide, terminal::Clear(terminal::ClearType::All))?;

            // begin rendering loop
            let mut selected: usize = 0;

            let selection: MenuOptions = 'app: loop {
                queue!(out, cursor::MoveTo(0, 0), terminal::Clear(terminal::ClearType::All))?;
                queue!(out, style::Print(title.clone()))?;
                queue!(out, style::Print("Use ↑/↓ to move, Esc to exit\n\n"))?;

                for (i, option) in MenuOptions::iter().enumerate() {
                    if i == selected {
                        queue!(out, style::SetAttribute(style::Attribute::Reverse))?;
                    }
                    let item = match option {
                        MenuOptions::NewGame(str) => str,
                        MenuOptions::Statistics(str) => str,
                        MenuOptions::Quit(str) => str
                    };

                    queue!(out, style::Print(format!(" {} {}\n", if i == selected { ">" } else { " " }, item)))?;
                    if i == selected {
                        queue!(out, style::SetAttribute(style::Attribute::NoReverse))?;
                    }
                }
                out.flush()?;

                // poll for the last event that occurred
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Press {
                        match key.code {
                            event::KeyCode::Up => selected = selected.saturating_sub(1),
                            event::KeyCode::Down => {
                                if selected + 1 < MenuOptions::ALL.len() {
                                    selected += 1;
                                }
                            }
                            // get the menu option selected by the user
                            event::KeyCode::Enter => {
                                match MenuOptions::ALL[selected] {
                                    MenuOptions::NewGame(_) => break 'app MenuOptions::NewGame(""),
                                    MenuOptions::Statistics(_) => break 'app MenuOptions::Statistics(""),
                                    MenuOptions::Quit(_) => break 'app MenuOptions::Quit(""),
                                    _ => break 'app MenuOptions::Quit("")  // needed to exhaust string patterns
                                }
                            }
                            event::KeyCode::Esc => break 'app MenuOptions::Quit(""),
                            _ => {}
                        }
                    }
                }          
            };

            execute!(out, cursor::Show, terminal::LeaveAlternateScreen)?;
            match terminal::disable_raw_mode() {
                Ok(_) => Ok(selection),
                Err(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed menu operation, exiting."))
            }
        }
    }
}

/// This module contains logic for managing board state.
mod board {
    use colored::Colorize;
    use std::fmt;
    use thiserror::Error;

    /// Constants for controlling the size of the game board.
    pub const ROWS: usize = 10;
    pub const COLS: usize = 10;
    pub const NUM_SHIPS: usize = 5;

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
    
    /// An enum that defines all possible states a board cell can exist in.
    /// When a cell is modified on the board, we simply adjust the enumeration
    /// assigned to that cell.
    #[derive(Clone, Copy)]
    pub enum Cell {
        Empty,
        Guessed,
        OwnShip(ShipType),
        OwnShipHit(ShipType),
        EnemyShipHit(ShipType),
        Highlighted
    }

    impl fmt::Display for Cell {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let cell_content = match self {
                Cell::Empty => "□".black(),
                Cell::Guessed => "▣".white(),
                Cell::OwnShip(_) => "◼".green(),
                Cell::OwnShipHit(_) => "◼".yellow(),
                Cell::EnemyShipHit(_) => "◼".red(),
                Cell::Highlighted => "◼".blue(),
            };
            write!(f, "{}", cell_content)
        }
    }

    /// A struct to contain all associated data with a ship. 
    pub struct Ship {
        pub ship_type: ShipType, 
        pub length: usize, 
        pub indices: std::vec::Vec<usize>,
        pub is_sunk: bool
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
                indices: vec![0; length],
                is_sunk: false
            }
        }

        // A helper function to return a new array containing all the initialized ships
        pub fn get_all_ships() -> [Ship; NUM_SHIPS] {
            [
                Ship::new(ShipType::Carrier),
                Ship::new(ShipType::Battleship),
                Ship::new(ShipType::Destroyer),
                Ship::new(ShipType::Submarine),
                Ship::new(ShipType::PatrolBoat)
            ]
        }
    }

    /// An enum for managing board errors.
    #[derive(Debug, Error)]
    pub enum BoardError {
        #[error("Indicated board index out of bounds!")]
        IndexOutOfBounds
    }

    /// A structure for encapsulating board state and logic.
    pub struct Board {
        cells: [Cell; ROWS * COLS]
    }

    impl Board {
        /// Generate a new board of empty cells.
        pub fn new_empty() -> Self {
            Self {
                cells: [Cell::Empty; ROWS * COLS]
            }
        }

        /// Update a single cell in the board.
        pub fn update(&mut self, row_idx: usize, col_idx: usize, new_state: Cell) {
            if (row_idx < ROWS) || (col_idx < COLS) {
                self.cells[row_idx * col_idx + col_idx] = new_state;
            }
        }

        /// Given a row and column index, return the Cell enum at that position.
        pub fn get(&self, r: usize, c: usize) -> Cell {
            self.cells[r * c + c]
        }
    }
}