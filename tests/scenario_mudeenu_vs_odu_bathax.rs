use std::path::PathBuf;

use chaostcg::engine::cards::*;
use chaostcg::engine::data_loader::{load_creatures, load_single};
use chaostcg::engine::logic::*;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn mudeenu_cactus_lightning_vs_odu_vlaric_at_mipedim_oasis() {
    // ---- Paths
    let creatures_dir   = root().join("data/cards/creatures");
    let cactus_path     = root().join("data/cards/battlegear/mipedian_cactus.json");
    let vlaric_path     = root().join("data/cards/battlegear/vlaric_shard.json");
    let lightning_path  = root().join("data/cards/attacks/lightning_burst.json");
    let oasis_path      = root().join("data/cards/locations/mipedim_oasis.json");

    // ---- Load cards
    let mudeenu = load_creatures(&creatures_dir)
        .into_iter()
        .find(|c| c.name == "Prince Mudeenu")
        .expect("Prince Mudeenu not found");
    let odu = load_creatures(&creatures_dir)
        .into_iter()
        .find(|c| c.name == "Odu-Bathax")
        .expect("Odu-Bathax not found");

    let cactus: BattleGearCard   = load_single(cactus_path);
    let vlaric: BattleGearCard   = load_single(vlaric_path);
    let lightning: AttackCard    = load_single(lightning_path);
    let oasis: LocationCard      = load_single(oasis_path);

    // ---- Runtime instances
    let mut att = CreatureInstance::from(&mudeenu);
    let mut def = CreatureInstance::from(&odu);

    // Sanity baselines (from JSON)
    assert_eq!(att.wisdom, 70);
    assert_eq!(def.power, 60);
    assert_eq!(att.current_energy, 45);
    assert_eq!(def.current_energy, 35);

    // ---- Equip gear
    equip_battlegear(&mut att, &cactus); // +15 Wisdom, conditional teleport (unused)
    equip_battlegear(&mut def, &vlaric); // Earth 5 grant

    // Check equips applied
    assert_eq!(att.wisdom, 70 + 15, "Mipedian Cactus should add +15 Wisdom");
    assert!(def.elements.contains(&Element::Earth), "Vlaric should grant Earth element");
    assert_eq!(*def.elemental_bonus.get(&Element::Earth).unwrap_or(&0), 5);

    // ---- Apply location (Mipedim Oasis)
    {
        let mut party = [&mut att, &mut def];
        apply_location(&mut party, &oasis);
    }

    // Oasis buffs only Mipedians on their first attack; no immediate energy change
    assert_eq!(att.current_energy, 45);
    assert_eq!(def.current_energy, 35);

    // ---- Compute damage (Lightning Burst)
    // Mudeenu has NO Air → Lightning Burst’s Air 5 and Air-only debuff DO NOT apply.
    let base = compute_attack_damage(&att, &def, &lightning);
    assert_eq!(base, 5, "Without Air, Lightning Burst should deal only base 5");

    // Location bonus: Mipedim Oasis = +10 on FIRST attack if attacker is Mipedian
    let loc_bonus = add_location_attack_bonus(&att, &def, &oasis);
    // Expect +10 (if this fails, see note below to extend add_location_attack_bonus)
    assert_eq!(loc_bonus, 10, "Mipedim Oasis should add +10 for Mipedians on first attack");

    let total = base + loc_bonus;
    assert_eq!(total, 15);

    // ---- Apply damage
    deal_damage(&mut def, total);
    assert_eq!(def.current_energy, 35 - 15);

    // ---- Air-only debuff from Lightning Burst should NOT trigger (attacker lacks Air)
    assert_eq!(def.power, 60, "No Air debuff: defender Power stays 60");
}