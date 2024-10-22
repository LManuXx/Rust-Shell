use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{self, stdout, Write};
use crate::modules::alias::AliasManager;
use crate::modules::command::autocomplete_command;
use crate::modules::history::CommandHistory;


pub fn read_input(alias_manager: &AliasManager, history: &mut CommandHistory) -> String {
    let stdin = io::stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
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
            Key::Ctrl('a') => {
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
            Key::Up => {
                if let Some(previous) = history.previous_command() {
                    input.clear();
                    input.push_str(previous);
                    write!(stdout, "\n> {}", input).unwrap();  // Solo el prompt y el input
                    stdout.flush().unwrap();
                }
            }
            Key::Down => {
                if let Some(next) = history.next_command() {
                    input.clear();
                    input.push_str(next);
                    write!(stdout, "\r\x1B[K> {}", input).unwrap();  // Solo el prompt y el input
                    stdout.flush().unwrap();
                }
            }
            _ => {}
        }
    }

    println!();
    input
}
