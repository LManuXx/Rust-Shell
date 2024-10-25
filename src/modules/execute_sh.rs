use std::fs::File;
use std::io::{self, BufRead};
use crate::modules::command::handle_command;
use crate::modules::config::Config;
use crate::modules::alias::AliasManager;

pub fn execute_script(
    file_path: &str,
    alias_manager: &mut AliasManager,
    config: &mut Config
) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let command = line?;
        if command.is_empty() {
            continue;
        }


        process_command(&command, alias_manager, config);
    }

    Ok(())
}

fn process_command(
    input: &str,
    alias_manager: &mut AliasManager,
    config: &mut Config,
) {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.is_empty() {
        return;
    }

    let command = tokens[0];
    let args = &tokens[1..];

    let resolved_command = alias_manager
        .resolve_alias(command)
        .cloned()
        .unwrap_or_else(|| command.to_string());

    handle_command(&resolved_command,args, alias_manager,config,);
}
