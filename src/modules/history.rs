use std::fs::{OpenOptions, File};
use std::io::{self, BufRead, BufReader, Write};

pub struct CommandHistory {
    pub history: Vec<String>,
    pub position: Option<usize>,
}

impl CommandHistory {
    pub fn new() -> Self {
        CommandHistory {
            history: Vec::new(),
            position: None,
        }
    }

    pub fn load_from_file(file_path: &str) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut history = Vec::new();

        for line in reader.lines() {
            history.push(line?);
        }

        Ok(CommandHistory {
            history,
            position: None,
        })
    }

    pub fn save_to_file(&self, file_path: &str) -> io::Result<()> {
        let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(file_path)?;
        for command in &self.history {
            writeln!(file, "{}", command)?;
        }
        Ok(())
    }

    pub fn add_command(&mut self, command: &str) {
        self.history.push(command.to_string());
        self.position = None;
    }

    pub fn previous_command(&mut self) -> Option<&String> {
        if let Some(pos) = self.position {
            if pos > 0 {
                self.position = Some(pos - 1);
            }
        } else {
            self.position = Some(self.history.len().saturating_sub(1));
        }
        self.position.and_then(|pos| self.history.get(pos))
    }

    pub fn next_command(&mut self) -> Option<&String> {
        if let Some(pos) = self.position {
            if pos < self.history.len().saturating_sub(1) {
                self.position = Some(pos + 1);
            } else {
                self.position = None;
            }
        }
        self.position.and_then(|pos| self.history.get(pos))
    }
}