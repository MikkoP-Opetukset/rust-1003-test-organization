//! Exercise public character behavior as a caller of the library would.

mod common;

#[test]
fn public_character_rules_work_together() {
    let mut player = common::player();
    player.take_damage(6, 2);
    player.heal(3);

    assert_eq!(player.hp(), 19);
}
