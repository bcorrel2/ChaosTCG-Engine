use chaostcg::engine::data_loader::load_single;
use chaostcg::engine::cards::{CreatureCard, BattleGearCard, AttackCard, LocationCard};
use chaostcg::engine::logic::{
    CreatureInstance,
    equip_battlegear,
    compute_attack_damage,
    add_location_attack_bonus,
    sacrifice_equipped_gear,
};

#[test]
fn heptadd_lyre_pebblestorm_vs_maxxor_at_forest_of_life() {
    // --- Load cards ---
    let heptadd: CreatureCard = load_single("data/cards/creatures/heptadd.json");
    let maxxor:  CreatureCard = load_single("data/cards/creatures/maxxor.json");
    let lyre:    BattleGearCard = load_single("data/cards/battlegear/mugicians_lyre.json");
    let pebble:  AttackCard = load_single("data/cards/attacks/pebblestorm.json");
    let forest:  LocationCard = load_single("data/cards/locations/forest_of_life.json");

    // --- Instances ---
    let mut att = CreatureInstance::from(&heptadd);
    let mut def = CreatureInstance::from(&maxxor);

    // Heptadd should start with 1 Mugic counter
    assert_eq!(att.mugic_counters, 1, "Heptadd should start with 1 Mugic counter");

    // Equip and then sacrifice Mugician’s Lyre
    equip_battlegear(&mut att, &lyre);
    assert!(att.equipped_gear.is_some(), "Lyre should be equipped");

    sacrifice_equipped_gear(&mut att);
    assert!(att.equipped_gear.is_none(), "Lyre should be removed after sacrifice");
    assert_eq!(att.mugic_counters, 2, "Lyre sacrifice should add +1 Mugic counter");

    // --- Attack: Pebblestorm ---
    // Pebblestorm: base 0 + 5 (Air) + 5 (Earth)
    let base = compute_attack_damage(&att, &def, &pebble);
    let loc  = add_location_attack_bonus(&att, &def, &forest); // Forest only heals later
    let total = base + loc;

    assert_eq!(base, 10, "Pebblestorm should deal 5 (Air) + 5 (Earth) = 10");
    assert_eq!(loc, 0, "Forest of Life should not modify attack damage");
    assert_eq!(total, 10, "Total immediate damage should be 10");
}