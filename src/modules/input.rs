use termion::event::Key;
use termion::input::TermRead;
use std::io::{self, Write};
use crate::modules::alias::AliasManager;
use crate::modules::command::autocomplete_command;



pub fn read_input(alias_manager: &AliasManager) -> String {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "> ").unwrap();
    stdout.flush().unwrap();

    let mut input = String::new();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('\n') => break,
            Key::Char(c) => {
                input.push(c);
                print!("{}", c);
                stdout.flush().unwrap();
            },
            Key::Backspace => {
                input.pop();
                print!("\r> {} ", input);
                stdout.flush().unwrap();
            },
            Key::Down => {
                // Lógica de autocompletado
                let mut matches = alias_manager.autocomplete_alias(&input);
                matches.append(&mut autocomplete_command(&input));

                if matches.len() == 1 {
                    input = matches[0].clone();
                    print!("\r> {}", input);
                } else if matches.len() > 1 {
                    println!("\nPosibles coincidencias: {:?}", matches);
                    print!("> {}", input);
                }

                stdout.flush().unwrap();
            },
            _ => {}
        }
    }

    println!();
    input
}
