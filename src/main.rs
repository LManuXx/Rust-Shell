mod modules {
    pub mod input;
    pub mod command;
    pub mod alias;
    pub mod config;
    pub mod history;
    pub mod prompt;
}

use modules::{alias::AliasManager, command::handle_command, config::Config, history::CommandHistory, prompt::print_prompt};
use std::path::Path;

fn main() {
    let mut alias_manager = AliasManager::new();
    let config_file = "config.json";

    let mut config = if Path::new(config_file).exists() {
        Config::load_from_file(config_file).unwrap()
    } else {
        Config::new("user") 
    };

    let mut history = CommandHistory::load_from_file("history.txt").unwrap_or_else(|_| CommandHistory::new());

    loop {
        print_prompt(&config);
        let input = modules::input::read_input(&alias_manager, &mut history, &mut config);

        if input.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = input.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        let command = tokens[0];
        let args = &tokens[1..];

        history.add_command(&input);
        history.save_to_file("history.txt").expect("Failed to save history");

        if !handle_command(command, args, &mut alias_manager, &mut config) {
            break;
        }
    }
}
