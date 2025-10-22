/// This module contains logic for managing menu state.
use std::io::Write;
use colored::Colorize;
use crossterm::{
    event,
    cursor,
    terminal,
    style,
    execute, queue
};

/// An enum defining all possible menu options.
pub enum MainMenuOptions {
    NewGame,
    Statistics,
    Quit
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

/// Reference to the .txt file containing the game title.
const TITLE: &str = include_str!("title.txt");

/// Display the main menu in the terminal.
pub fn show_main_menu() -> std::io::Result<MainMenuOptions>{
    // color the title string for the menu
    let title: colored::ColoredString = format!("{}\n\n", TITLE).red();

    // enter an alternate screen for the main menu
    terminal::enable_raw_mode()?;
    let mut out = std::io::stdout();
    execute!(out, terminal::EnterAlternateScreen, cursor::Hide, terminal::Clear(terminal::ClearType::All))?;

    // begin rendering loop
    let mut selected: usize = 0;
    let selection: MainMenuOptions = 'app: loop {
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

            let text = match option {
                MainMenuOptions::NewGame => "New Game",
                MainMenuOptions::Statistics => "Stats",
                MainMenuOptions::Quit => "Quit Game"
            };

            // print a right facing arrow on the selected option. print each options's text
            queue!(out, style::Print(format!(" {} {}\n", if i == selected { ">" } else { " " }, text)))?;

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
                    // rem_euclid always returns a positive int, so it handles negatives natively
                    event::KeyCode::Up => selected = (selected -1).rem_euclid(MainMenuOptions::ALL.len()),
                    event::KeyCode::Down => selected = (selected + 1) % MainMenuOptions::ALL.len(),

                    // get the menu option selected by the user and return it
                    event::KeyCode::Enter => {
                        match MainMenuOptions::ALL[selected] {
                            MainMenuOptions::NewGame => break 'app MainMenuOptions::NewGame,
                            MainMenuOptions::Statistics => break 'app MainMenuOptions::Statistics,
                            MainMenuOptions::Quit => break 'app MainMenuOptions::Quit,
                            _ => break 'app MainMenuOptions::Quit  // needed to exhaust string patterns
                        }
                    }

                    // quit game if the user hits Esc
                    event::KeyCode::Esc => break 'app MainMenuOptions::Quit,
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
