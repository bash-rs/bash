// Alias struct, alias table, and functions like add_alias, find_alias etc.

// This file defines the Alias struct and its associated methods for managing aliases in the core-rs library.
// We derive Debug for easy printing and debugging, Clone to allow copying, and PartialEq, Eq, and Hash traits
// for using Alias as a key in a HashMap, and comparison of Alias instances.
// The struct contains three fields:
// name, value, and flags.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Alias {
    pub name: String,
    pub value: String,
    pub flags: u8,
}
