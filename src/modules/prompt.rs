// modules/prompt.rs
use crate::modules::config::Config;
use std::io::{self, Write};

pub fn print_prompt(config: &Config) {
    print!("{}> ", config.username);
    io::stdout().flush().unwrap(); // Asegura que se imprime en la terminal inmediatamente
}
