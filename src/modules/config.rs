use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub username: String,
}

impl Config {
    pub fn new(username: &str) -> Self {
        Config {
            username: username.to_string(),
        }
    }

    pub fn load_from_file(filename: &str) -> Option<Self> {
        let data = fs::read_to_string(filename).ok()?;
        serde_json::from_str(&data).ok()
    }

    pub fn save_to_file(&self, filename: &str) {
        let data = serde_json::to_string(self).unwrap();
        fs::write(filename, data).expect("Unable to write file");
    }
}
