use std::path::PathBuf;

use chaostcg::engine::data_loader::{load_creatures, load_single};
use chaostcg::engine::cards::*;
use chaostcg::engine::logic::*;

/// Helper: build absolute paths from the crate root
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn maxxor_ring_pebblestorm_vs_chaor_whepcrack_at_kiru_city() {
    // --- File paths ---
    let creatures_dir = root().join("data/cards/creatures");
    let ring_path     = root().join("data/cards/battlegear/ring_of_naarin.json");
    let whepcrack_path= root().join("data/cards/battlegear/whepcrack.json");
    let pebble_path   = root().join("data/cards/attacks/pebblestorm.json");
    let kiru_path     = root().join("data/cards/locations/kiru_city.json");

    // --- Load cards ---
    let maxxor = load_creatures(&creatures_dir)
        .into_iter()
        .find(|c| c.name == "Maxxor")
        .expect("Maxxor not found");

    let chaor = load_creatures(&creatures_dir)
        .into_iter()
        .find(|c| c.name == "Chaor")
        .expect("Chaor not found");

    let ring_of_naarin: BattleGearCard = load_single(ring_path);
    let whepcrack:      BattleGearCard = load_single(whepcrack_path);
    let pebblestorm:    AttackCard     = load_single(pebble_path);
    let kiru_city:      LocationCard   = load_single(kiru_path);

    // --- Create runtime instances ---
    let mut maxx = CreatureInstance::from(&maxxor);
    let mut chao = CreatureInstance::from(&chaor);

    // --- Equip BattleGear ---
    equip_battlegear(&mut maxx, &ring_of_naarin);
    equip_battlegear(&mut chao, &whepcrack);

    // --- Apply Location effects ---
    let mut party = [&mut maxx, &mut chao];
    apply_location(&mut party, &kiru_city);

    // --- Check resulting stats ---
    // Ring of Na'arin: +10 Power, +10 Wisdom, +10 Energy if Courage ≥ 50
    // Kiru City: +10 Energy to OverWorlders
    assert_eq!(maxx.power,   65 + 10);
    assert_eq!(maxx.wisdom,  80 + 10);
    assert_eq!(maxx.courage, 100);
    assert_eq!(maxx.current_energy, 60 + 10 + 10); // base + ring + kiru city

    // Whepcrack: +15 Power
    assert_eq!(chao.power, 90 + 15);

    // --- Compute Pebblestorm damage ---
    // Pebblestorm: Base 0 + Earth 5 (Maxxor has Earth) = 5 total
    let dmg = compute_attack_damage(&maxx, &chao, &pebblestorm);
    assert_eq!(dmg, 5);

    // --- Deal damage and verify ---
    let pre_energy = chao.current_energy;
    deal_damage(&mut chao, dmg);
    assert_eq!(chao.current_energy, pre_energy - 5);
}

#[test]
fn chaor_whepcrack_flame_orb_vs_maxxor_ring_at_underworld_city() {
    // --- Paths ---
    let creatures_dir  = root().join("data/cards/creatures");
    let whepcrack_path = root().join("data/cards/battlegear/whepcrack.json");
    let ring_path      = root().join("data/cards/battlegear/ring_of_naarin.json");
    let flame_orb_path = root().join("data/cards/attacks/flame_orb.json");
    let uw_city_path   = root().join("data/cards/locations/underworld_city.json");

    // --- Load cards ---
    let chaor = load_creatures(&creatures_dir)
        .into_iter()
        .find(|c| c.name == "Chaor")
        .expect("Chaor not found");
    let maxxor = load_creatures(&creatures_dir)
        .into_iter()
        .find(|c| c.name == "Maxxor")
        .expect("Maxxor not found");

    let whepcrack:   BattleGearCard = load_single(whepcrack_path);
    let ring_naarin: BattleGearCard = load_single(ring_path);
    let flame_orb:   AttackCard     = load_single(flame_orb_path);
    let uw_city:     LocationCard   = load_single(uw_city_path);

    // --- Runtime instances ---
    let mut att = CreatureInstance::from(&chaor);   // attacker: Chaor
    let mut def = CreatureInstance::from(&maxxor);  // defender: Maxxor

    // --- Equip ---
    equip_battlegear(&mut att, &whepcrack);   // +15 Power, Fire 5 (elemental bonus)
    equip_battlegear(&mut def, &ring_naarin); // +10 Power, +10 Wisdom, +10 Energy (if Courage ≥ 50)

    // --- Apply location continuous effects (UW City has none; Kiru City was the +Energy one) ---
    let mut party = [&mut att, &mut def];
    apply_location(&mut party, &uw_city);

    // Defender’s starting energy: Maxxor 60 base +10 Ring = 70 (UW City does not buff)
    assert_eq!(def.current_energy, 70);

    // --- Compute damage ---
    // Flame Orb: base 5 + fire 5; Chaor gains Fire 5 from Whepcrack → +5 more fire
    //            Attack stat check: Power ≥ 75 → +10 (Chaor power 90+15=105)
    //            UnderWorld City challenge: Power ≥ 15 → +5 (Chaor is UnderWorlders)
    let base = compute_attack_damage(&att, &def, &flame_orb);
    let loc_bonus = add_location_attack_bonus(&att, &def, &uw_city);
    let total = base + loc_bonus;

    assert_eq!(base, 25, "Expected 5 (base) + 5 (fire) + 5 (Fire 5) + 10 (stat check)");
    assert_eq!(loc_bonus, 5, "UW City challenge should add +5");
    assert_eq!(total, 30);

    // --- Apply damage ---
    deal_damage(&mut def, total);
    assert_eq!(def.current_energy, 70 - 30); // => 40
}