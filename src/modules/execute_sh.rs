use std::fs::File;
use std::io::{self, BufRead};
use crate::modules::command::handle_command;
use crate::modules::config::Config;
use crate::modules::history::CommandHistory;
use crate::modules::alias::AliasManager;

pub fn execute_script(
    file_path: &str,
    alias_manager: &mut AliasManager,
    config: &mut Config,
    history: &mut CommandHistory
) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let command = line?;
        if command.is_empty() {
            continue;
        }

        history.add_command(&command);

        process_command(&command, alias_manager, config, history);
    }

    Ok(())
}

fn process_command(
    input: &str,
    alias_manager: &mut AliasManager,
    config: &mut Config,
    history: &mut CommandHistory
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

        history.add_command(&input);
        history.save_to_file("history.txt").expect("Failed to save history");
    handle_command(&resolved_command,args, alias_manager,config,);
}
