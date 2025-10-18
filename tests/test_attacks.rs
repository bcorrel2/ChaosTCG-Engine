use chaostcg::engine::data_loader::load_single;
use chaostcg::engine::cards::{AttackCard, CreatureCard};
use chaostcg::engine::logic::{CreatureInstance, compute_attack_damage};

fn heptadd() -> CreatureCard {
    // 50 Courage / 50 Power / 50 Wisdom / 50 Speed / 50 Energy
    //    Fire    /   Air    /   Earth   /  Water
    load_single("data/cards/creatures/heptadd.json")
}

fn maxxor() -> CreatureCard {
    // 100 Courage / 65 Power / 80 Wisdom / 50 Speed / 60 Energy
    load_single("data/cards/creatures/maxxor.json")
}

fn chaor() -> CreatureCard {
    // 95 Courage / 90 Power / 70 Wisdom / 60 Speed / 70 Energy
    load_single("data/cards/creatures/chaor.json")
}

/// Pebblestorm
/// base 0, Fire 0, Air 5, Earth 5, Water 0
/// No Special Effect.
#[test]
fn pebblestorm_test() {
    let atk_card: AttackCard = load_single("data/cards/attacks/pebblestorm.json");
    let att: CreatureInstance = CreatureInstance::from(&heptadd());
    let def: CreatureInstance = CreatureInstance::from(&maxxor());

    let dmg = compute_attack_damage(&att, &def, &atk_card);
    assert_eq!(dmg, 10, "Pebblestorm should deal 5 (Air) + 5 (Earth) = 10");

    // Sanity Test - Ensures Elemental Damage is Selectively Applied
    let att: CreatureInstance = CreatureInstance::from(&maxxor());
    let def: CreatureInstance = CreatureInstance::from(&heptadd());

    let dmg = compute_attack_damage(&att, &def, &atk_card);
    assert_eq!(dmg, 5, "With only Earth, Pebblestorm should deal 5 damage");
}

/// Flame Orb
/// base 5, Fire 5, Air 0, Earth 0, Water 0
/// No Special Effect.
#[test]
fn flame_orb_test() {
    let atk_card: AttackCard = load_single("data/cards/attacks/flame_orb.json");
    let att: CreatureInstance = CreatureInstance::from(&heptadd());
    let def: CreatureInstance = CreatureInstance::from(&maxxor());

    let dmg = compute_attack_damage(&att, &def, &atk_card);

    assert_eq!(dmg, 10, "Flame Orb should deal 5 (base) + 5 (fire) = 10 total damage");

    // Test Ability - Deal 10 Damage
    let att: CreatureInstance = CreatureInstance::from(&chaor());
    let def: CreatureInstance = CreatureInstance::from(&maxxor());

    let dmg = compute_attack_damage(&att, &def, &atk_card);

    assert_eq!(dmg, 20, "Flame Orb should deal 5 (base) + 5 (fire) + 10 (ability) = 20 total damage");
}
