use battleship::{
    game::{
        self,
        layouts::{self, TerminalLayout, menus::{self, new_game_menu::NewGameMenuOptions}}
    }
};

fn main() {
    // the very first thing we want to do is show the user the
    // main menu
    let game_type_option: Option<NewGameMenuOptions> = 'showingMenus: loop {
        let option = menus::main_menu::MainMenu::show();

        // check if the user asked to quit the game and early return
        if let Ok(menus::main_menu::MainMenuOptions::Quit) = option {
            println!("Thanks for playing!");
            break 'showingMenus None
        } else if let Err(_) = option {
            panic!("Unexpected error encountered, exiting the game.")
        }

        // if the new game menu selected, send them to the new game screen
        if let Ok(menus::main_menu::MainMenuOptions::NewGame) = option {
            match menus::new_game_menu::NewGameMenu::show() {
                Ok(menus::new_game_menu::NewGameMenuOptions::PlayComputer) => break 'showingMenus Some(menus::new_game_menu::NewGameMenuOptions::PlayComputer),
                Ok(menus::new_game_menu::NewGameMenuOptions::JoinGame) => print!("Joining a game"),
                Ok(menus::new_game_menu::NewGameMenuOptions::HostGame) => print!("Hosting a game"),
                Ok(menus::new_game_menu::NewGameMenuOptions::Back) => { /* do nothing; just go back to main menu loop */ },
                Err(_) => panic!("Unexpected error encountered, exiting the game.")
            }
        }   
    };

    // at this stage, we can begin the game!
    match game_type_option.unwrap() {
        menus::new_game_menu::NewGameMenuOptions::PlayComputer => {
            // create a new game against the computer
            let mut player = game::components::player::Player::new("Player");
            let mut computer_player = game::components::player::Player::new("Computer");

            // let the player set up their board
            layouts::game::board_setup::show(&mut player).expect("Failed to setup player ships");

            // setup the computer's board automatically
            computer_player.auto_place_ships(100, 10).expect("Failed to auto-place computer ships");

            // start the game loop
            
            let mut game_instance = game::game::Game::new(player, computer_player);
            
            match game_instance.start_loop() {
                Ok(winner) => {
                    layouts::game::win_screen::show(game_instance.get_player_a(), game_instance.get_player_b(),
                    match winner {
                        game::game::GameEndReason::PlayerAWon => "Player",
                        game::game::GameEndReason::PlayerBWon => "Computer"
                    }
                ).expect("Failed to show win screen");
                println!("Thanks for playing!");
            },
                Err(e) => println!("Game ended with error: {}", e)
            }
        },
        _ => {}  // for now, we only support  playing the computer
    }
}
