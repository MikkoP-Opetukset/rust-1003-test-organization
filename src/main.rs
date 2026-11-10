//! Entry point for Chapter 11.3: Test Organization.
//!
//! The game rules and parser live in the library so unit tests, integration
//! tests, and documentation examples can exercise them.

use rust_1003_test_organization::{Character, parse_command};

fn main() {
    let command = parse_command("go north").expect("the example command is valid");
    println!("Parsed command: {command:?}");

    let mut player = Character::new("Aria", 20);
    player.take_damage(6, 2);
    println!("After taking damage: {player:?}");
}
