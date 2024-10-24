use std::process::Command;
use std::env;
use std::path::Path;
use std::fs;
use crate::modules::alias::AliasManager;
use crate::modules::config::Config;

pub fn handle_command(command: &str, args: &[&str], alias_manager: &mut AliasManager, config: &mut Config) -> bool {
    if command == "exit" {
        return false;
    }

    if command == "alias" {
        if args.len() != 2 {
            eprintln!("Usage: alias <name> <command>");
        } else {
            alias_manager.add_alias(args[0], args[1]);
        }
        return true;
    }

    if command == "setprompt" {
        if args.len() != 1 {
            eprintln!("Usage: setprompt <username>");
        } else {
            config.username = args[0].to_string();
            config.save_to_file("config.json");
            print!("\r");
        }
        return true;
    }

    if command == "cd" {
        if args.is_empty() {
            eprintln!("cd: missing argument");
        } else {
            let new_dir = Path::new(args[0]);
            print!("\r");
            if let Err(e) = env::set_current_dir(&new_dir) {
                eprintln!("cd: {}", e);
            }
        }
        return true;
    }

    execute_command(command, args);
    true
}

pub fn execute_command(command: &str, args: &[&str]) {
    let child = Command::new(command)
        .args(args)
        .spawn();

    match child {
        Ok(mut process) => {
            process.wait().expect("Command wasn't running");
        }
        Err(e) => {
            eprintln!("Error running command: {}", e);
        }
    }
}

pub fn autocomplete_command(partial: &str) -> Vec<String> {
    let paths = ["/bin", "/usr/bin", "/usr/local/bin"];
    let mut matches = Vec::new();

    for path in &paths {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    let command = file_name.to_string_lossy();
                    if command.starts_with(partial) {
                        matches.push(command.to_string());
                    }
                }
            }
        }
    }

    matches
}
