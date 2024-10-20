use std::collections::HashMap;

pub struct AliasManager {
    aliases: HashMap<String, String>,
}

impl AliasManager {
    
    pub fn new() -> AliasManager {
        AliasManager {
            aliases: HashMap::new(),
        }
    }

    pub fn add_alias(&mut self, alias: &str, command: &str) {
        self.aliases.insert(alias.to_string(), command.to_string());
    }

    pub fn resolve_alias(&self, alias: &str) -> Option<&String> {
        self.aliases.get(alias)
    }
}
