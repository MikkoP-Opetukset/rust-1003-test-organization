/// A player whose health is bounded by a maximum.
#[derive(Debug, PartialEq, Eq)]
pub struct Character {
    name: String,
    hp: u32,
    max_hp: u32,
}

impl Character {
    /// Create a character at full health.
    pub fn new(name: impl Into<String>, max_hp: u32) -> Self {
        Self {
            name: name.into(),
            hp: max_hp,
            max_hp,
        }
    }

    /// The character's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The character's current health.
    pub fn hp(&self) -> u32 {
        self.hp
    }

    /// The character's maximum health.
    pub fn max_hp(&self) -> u32 {
        self.max_hp
    }

    /// Apply damage after armor, keeping a player at a minimum of one health.
    pub fn take_damage(&mut self, incoming: u32, armor: u32) {
        self.hp = self
            .hp
            .saturating_sub(damage_after_armor(incoming, armor))
            .max(1);
    }

    /// Restore health without allowing it to exceed the maximum.
    pub fn heal(&mut self, points: u32) {
        self.hp = self.hp.saturating_add(points).min(self.max_hp);
    }
}

/// Armor reduces a hit, but every hit deals at least one point of damage.
fn damage_after_armor(incoming: u32, armor: u32) -> u32 {
    incoming.saturating_sub(armor).max(1)
}

#[cfg(test)]
mod tests {
    // These colocated unit tests can inspect private implementation details.
    use super::*;

    #[test]
    fn private_damage_rule_never_returns_zero() {
        assert_eq!(damage_after_armor(3, 100), 1);
    }

    #[test]
    fn healing_stops_at_maximum_health() {
        let mut player = Character::new("Aria", 20);
        player.take_damage(6, 2);

        player.heal(10);

        assert_eq!(player.hp(), 20);
    }
}
