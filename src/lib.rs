//! Rules and commands for a small dungeon game.
//!
//! The library target makes the public API available to integration tests and
//! documentation tests. The binary in `main.rs` is a thin caller of this code.

mod command;
mod rules;

// Keep the modules private and expose only the API the binary and external
// callers need. Unit tests remain inside their modules to test private rules.
pub use command::{Command, Direction, parse_command};
pub use rules::Character;
