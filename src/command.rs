/// A direction a player can move.
#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// A command understood by the dungeon rules.
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Go(Direction),
    Heal(u32),
}

/// Parse a command such as `go north` or `heal 10`.
///
/// ```
/// use rust_1003_test_organization::{Command, Direction, parse_command};
///
/// let command = parse_command("go north").unwrap();
/// assert_eq!(command, Command::Go(Direction::North));
/// ```
///
/// The returned `Result` can also be handled without panicking:
///
/// ```
/// use rust_1003_test_organization::{Command, parse_command};
///
/// match parse_command("heal 10") {
///     Ok(command) => assert_eq!(command, Command::Heal(10)),
///     Err(error) => panic!("valid example was rejected: {error}"),
/// }
/// ```
pub fn parse_command(input: &str) -> Result<Command, &'static str> {
    let mut words = input.split_whitespace();
    let verb = words.next().ok_or("command cannot be empty")?;

    if verb.eq_ignore_ascii_case("go") {
        let direction = words.next().ok_or("go needs a direction")?;
        if words.next().is_some() {
            return Err("go takes one direction");
        }
        return parse_direction(direction)
            .map(Command::Go)
            .ok_or("unknown direction");
    }

    if verb.eq_ignore_ascii_case("heal") {
        let points = words
            .next()
            .ok_or("heal needs an amount")?
            .parse()
            .map_err(|_| "healing amount must be a whole number")?;
        if words.next().is_some() {
            return Err("heal takes one amount");
        }
        return Ok(Command::Heal(points));
    }

    Err("unknown command")
}

/// Convert the user's text to one of the directions the game supports.
fn parse_direction(text: &str) -> Option<Direction> {
    if text.eq_ignore_ascii_case("north") {
        Some(Direction::North)
    } else if text.eq_ignore_ascii_case("south") {
        Some(Direction::South)
    } else if text.eq_ignore_ascii_case("east") {
        Some(Direction::East)
    } else if text.eq_ignore_ascii_case("west") {
        Some(Direction::West)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    // As a child module, these tests can also call the private
    // `parse_direction` helper.
    use super::*;

    #[test]
    fn private_direction_parser_accepts_mixed_case() {
        assert_eq!(parse_direction("NoRtH"), Some(Direction::North));
    }

    #[test]
    fn parser_rejects_an_unsupported_direction() {
        assert_eq!(parse_command("go upward"), Err("unknown direction"));
    }
}
