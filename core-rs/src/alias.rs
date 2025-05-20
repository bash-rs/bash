// alias.rs -- Rust port of Bash alias management functionality
// This is a simplified, idiomatic Rust version of the C alias.c logic.

use std::collections::HashMap;
use std::sync::Mutex;

bitflags::bitflags! {
    #[derive(Debug, Clone)]
    pub struct AliasFlags: u8 {
        const NONE = 0;
        const EXPAND_NEXT = 0b0000_0001;
        const BEING_EXPANDED = 0b0000_0010;
    }
}

#[derive(Debug, Clone)]
pub struct Alias {
    pub name: String,
    pub value: String,
    pub flags: AliasFlags,
}

pub struct AliasManager {
    aliases: Mutex<HashMap<String, Alias>>,
}

impl AliasManager {
    pub fn new() -> Self {
        AliasManager {
            aliases: Mutex::new(HashMap::new()),
        }
    }

    pub fn find_alias(&self, name: &str) -> Option<Alias> {
        let aliases = self.aliases.lock().unwrap();
        aliases.get(name).cloned()
    }

    pub fn get_alias_value(&self, name: &str) -> Option<String> {
        self.find_alias(name).map(|a| a.value)
    }

    pub fn add_alias(&self, name: &str, value: &str) {
        let mut aliases = self.aliases.lock().unwrap();
        let mut flags = AliasFlags::NONE;
        if let Some(last) = value.chars().last() {
            if last == ' ' || last == '\t' {
                flags |= AliasFlags::EXPAND_NEXT;
            }
        }
        let alias = Alias {
            name: name.to_string(),
            value: value.to_string(),
            flags,
        };
        aliases.insert(name.to_string(), alias);
    }

    pub fn remove_alias(&self, name: &str) -> bool {
        let mut aliases = self.aliases.lock().unwrap();
        aliases.remove(name).is_some()
    }

    pub fn delete_all_aliases(&self) {
        let mut aliases = self.aliases.lock().unwrap();
        aliases.clear();
    }

    pub fn all_aliases(&self) -> Vec<Alias> {
        let aliases = self.aliases.lock().unwrap();
        let mut list: Vec<_> = aliases.values().cloned().collect();
        list.sort_by(|a, b| a.name.cmp(&b.name));
        list
    }
}

impl Default for AliasManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alias_manager() {
        let manager = AliasManager::new();
        manager.add_alias("ll", "ls -l ");
        assert_eq!(manager.get_alias_value("ll"), Some("ls -l ".to_string()));
        assert!(
            manager
                .find_alias("ll")
                .unwrap()
                .flags
                .contains(AliasFlags::EXPAND_NEXT)
        );
        manager.remove_alias("ll");
        assert!(manager.get_alias_value("ll").is_none());
    }
}
