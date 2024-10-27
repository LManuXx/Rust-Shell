use std::process::Command;
use std::env;
use std::path::Path;
use crate::modules::alias::AliasManager;
use crate::modules::config::Config;
use crate::modules::system_info::show_sys_info;

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

    if command == "sysinfo" {
        show_sys_info();
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


