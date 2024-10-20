use std::process::Command;

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
