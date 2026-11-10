use rust_1003_test_organization::Character;

/// Shared setup used by the package's integration tests.
pub fn player() -> Character {
    Character::new("Aria", 20)
}
