//! Integration tests compile as external crates and use only the public API.

mod common;

use rust_1003_test_organization::{Command, Direction, parse_command};

#[test]
fn parses_a_valid_movement_command() {
    let command = parse_command("go north").expect("the example command is valid");

    assert_eq!(command, Command::Go(Direction::North));
}

#[test]
fn command_words_are_case_insensitive() {
    let command = parse_command("GO WEST").expect("the command is valid");

    assert_eq!(command, Command::Go(Direction::West));
}

#[test]
fn rejects_unknown_and_empty_commands() {
    assert_eq!(parse_command("dance"), Err("unknown command"));
    assert_eq!(parse_command(""), Err("command cannot be empty"));
}

#[test]
fn integration_test_can_share_setup() {
    let player = common::player();

    assert_eq!(player.name(), "Aria");
    assert_eq!(player.hp(), player.max_hp());
}
