use std::process::Command;
use std::env;
use std::path::Path;


    pub fn execute_command(command: &str, args: &[&str]) {

        if command == "cd" {
            if args.is_empty() {
                eprintln!("cd: missing argument");
            } else {
                let new_dir = Path::new(args[0]);
                if let Err(e) = env::set_current_dir(&new_dir) {
                    eprintln!("cd: {}", e);
                }
            }
            return;
        }

        let mut child = Command::new(command)
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

