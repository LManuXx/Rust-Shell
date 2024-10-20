mod modules {
    pub mod input;
    pub mod command;
    pub mod alias;
    pub mod config;
    pub mod history;
}

use modules::{alias::AliasManager, command::execute_command, config::Config, history::CommandHistory};
use std::path::Path;


fn main() {
    let mut alias_manager = AliasManager::new();
    let config_file = "config.json";

    let mut config = if Path::new(config_file).exists() {
        Config::load_from_file(config_file).unwrap()
    } else {
        Config::new("user")  // Usuario por defecto
    };

    let mut history = CommandHistory::load_from_file("history.txt").unwrap_or_else(|_| CommandHistory::new());


    loop {
        print!("{}> ", config.username);
        let input = modules::input::read_input(&alias_manager,&mut history);

        if input == "exit" {
            break;
        }

        let tokens: Vec<&str> = input.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        let command = tokens[0];
        let args = &tokens[1..];

        history.add_command(&input);
        history.save_to_file("history.txt").expect("Failed to save history");

        if command == "alias" {
            if args.len() != 2 {
                eprintln!("Usage: alias <name> <command>");
            } else {
                alias_manager.add_alias(args[0], args[1]);
            }
            continue;
        }

        let resolved_command = alias_manager
            .resolve_alias(command)
            .cloned()
            .unwrap_or_else(|| command.to_string());

        execute_command(&resolved_command, args, &mut config);
    }
}

