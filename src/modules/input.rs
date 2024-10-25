use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{self, stdout, Write};
use crate::modules::history::CommandHistory;
use crate::modules::prompt::print_prompt;
use crate::modules::config::Config;


pub fn read_input(history: &mut CommandHistory, config:&Config) -> String {
    let stdin = io::stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    stdout.flush().unwrap();

    let mut input = String::new();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('\n') => {
                if input.trim().is_empty() {
                    print!("\r");
                    break;
                } else {
                    break;
                }
            },
            Key::Char(c) => {
                input.push(c);
                print!("{}", c);
                stdout.flush().unwrap();
            },
            Key::Backspace => {
                if !input.is_empty() {
                    input.pop(); 
                    print!("\r");
                    print_prompt(&config);
                    print!("{}", input);
                    print!(" \r");
                    print_prompt(&config);
                    print!("{}", input);
            
                    stdout.flush().unwrap();
                }
            },
            Key::Up => {
                if let Some(previous) = history.previous_command() {
                    input.clear();
                    input.push_str(previous);
                    print!("\r");
                    print_prompt(&config);
                    print!("\x1B[K");
                    print!("{}", input);
                    stdout.flush().unwrap();
                }
            }
            Key::Down => {
                if let Some(next) = history.next_command() {
                    input.clear();
                    input.push_str(next);
                    print!("\r");
                    print_prompt(&config);
                    print!("\x1B[K");
                    print!("{}", input);
                    stdout.flush().unwrap();
                }
            }
            _ => {}
        }
    }

    println!();
    input
}
