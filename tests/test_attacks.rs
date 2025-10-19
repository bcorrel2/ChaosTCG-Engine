use chaostcg::engine::data_loader::load_single;
use chaostcg::engine::cards::{AttackCard, BattleGearCard, CreatureCard};
use chaostcg::engine::logic::{CreatureInstance, resolve_attack_and_apply_effects, Board, Side, equip_battlegear};


pub fn board() -> Board {
    Board {
        active_attacker_idx: 0,
        active_defender_idx: 0,
        attacker: Side {
            creatures: Vec::new(),
        },
        defender: Side {
            creatures: Vec::new(),
        },
    }
}

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
    let mut board = board();

    let atk_card: AttackCard = load_single("data/cards/attacks/pebblestorm.json");
    let att: CreatureInstance = CreatureInstance::from(&heptadd());
    let def: CreatureInstance = CreatureInstance::from(&maxxor());

    // push creatures (moves them into the board)
    board.attacker.creatures.push(att);
    board.defender.creatures.push(def);

    let dmg = resolve_attack_and_apply_effects(0, 0, &atk_card, &mut board);
    assert_eq!(dmg, 10, "Pebblestorm should deal 5 (Air) + 5 (Earth) = 10");

    // Sanity Test - Ensures Elemental Damage is Selectively Applied
    let mut board = crate::board();

    let att: CreatureInstance = CreatureInstance::from(&maxxor());
    let def: CreatureInstance = CreatureInstance::from(&heptadd());

    // change out attacking and defending creatures
    board.attacker.creatures.push(att);
    board.defender.creatures.push(def);

    let dmg = resolve_attack_and_apply_effects(0, 0, &atk_card, &mut board);
    assert_eq!(dmg, 5, "With only Earth, Pebblestorm should deal 5 damage");
}

/// Flame Orb
/// base 5, Fire 5, Air 0, Earth 0, Water 0
/// No Special Effect.
#[test]
fn flame_orb_test() {
    let mut board = board();

    let atk_card: AttackCard = load_single("data/cards/attacks/flame_orb.json");
    let att: CreatureInstance = CreatureInstance::from(&heptadd());
    let def: CreatureInstance = CreatureInstance::from(&maxxor());

    // push creatures (moves them into the board)
    board.attacker.creatures.push(att);
    board.defender.creatures.push(def);

    let dmg = resolve_attack_and_apply_effects(0, 0, &atk_card, &mut board);

    assert_eq!(dmg, 10, "Flame Orb should deal 5 (base) + 5 (fire) = 10 total damage");



    // Test Ability - Deal 10 Damage
    let att: CreatureInstance = CreatureInstance::from(&chaor());

    // change out attacking creature
    board.attacker.creatures.pop();
    board.attacker.creatures.push(att);

    let dmg = resolve_attack_and_apply_effects(0, 0, &atk_card, &mut board);

    assert_eq!(dmg, 20, "Flame Orb should deal 5 (base) + 5 (fire) + 10 (ability) = 20 total damage");
}

// Lightning Burst
/// base 5, Fire 0, Air 5, Earth 0, Water 0
/// Air: Opposing Creature loses 25 Power until the end of the turn.
#[test]
fn lightning_burst_test() {
    let mut board = board();

    let atk_card: AttackCard = load_single("data/cards/attacks/lightning_burst.json");
    let att: CreatureInstance = CreatureInstance::from(&heptadd());
    let def: CreatureInstance = CreatureInstance::from(&maxxor());

    // push creatures (moves them into the board)
    board.attacker.creatures.push(att);
    board.defender.creatures.push(def);

    let dmg = resolve_attack_and_apply_effects(0, 0, &atk_card, &mut board);

    // damage check
    assert_eq!(dmg, 10, "Lightning should deal 5 (base) + 5 (air) = 10 total damage");

    let def_after = &board.defender.creatures[0];
    assert_eq!(
        def_after.power, 40,
        "Lightning Burst's effect should reduce defender Power by 25 (65 → 40)"
    );
}

// Windslash
/// base 5, Fire 0, Air 0, Earth 0, Water 0
/// Target opponent turns all face-down BattleGear Cards they control face up;
#[test]
fn windslash_test() {
    let mut board = board();

    let atk_card: AttackCard = load_single("data/cards/attacks/windslash.json");
    let att: CreatureInstance = CreatureInstance::from(&heptadd());
    let mut def: CreatureInstance = CreatureInstance::from(&maxxor());
    let mut extra1: CreatureInstance = CreatureInstance::from(&chaor());
    let mut extra2: CreatureInstance = CreatureInstance::from(&heptadd());

    let lyre: BattleGearCard   = load_single("data/cards/battlegear/mugicians_lyre.json");
    let whep: BattleGearCard   = load_single("data/cards/battlegear/whepcrack.json");
    let ring: BattleGearCard   = load_single("data/cards/battlegear/ring_of_naarin.json");

    equip_battlegear(&mut def, &ring, &mut board);
    equip_battlegear(&mut extra1, &whep, &mut board);
    equip_battlegear(&mut extra2, &lyre, &mut board);

    // push creatures (moves them into the board)
    board.attacker.creatures.push(att);
    board.defender.creatures.push(def);
    board.defender.creatures.push(extra1);
    board.defender.creatures.push(extra2);

    let dmg = resolve_attack_and_apply_effects(0, 0, &atk_card, &mut board);

    assert_eq!(dmg, 5, "Windslash should deal 5 (base) total damage.");

    // Test Ability - BattleGear Revealed
    for (i, c) in board.defender.creatures.iter().enumerate() {
        let revealed = c.equipped_gear.as_ref().map(|g| g.revealed).unwrap_or(false);
        assert!(revealed, "Defender creature {}'s Battlegear should be revealed by Windslash", i);
    }
}

/// Lavalanche
/// base 10, Fire 5, Air 0, Earth 5, Water 0
/// No Special Effect.
#[test]
fn lavalanche_test() {
    let mut board = board();

    let atk_card: AttackCard = load_single("data/cards/attacks/lavalanche.json");
    let att: CreatureInstance = CreatureInstance::from(&heptadd());
    let def: CreatureInstance = CreatureInstance::from(&maxxor());

    // push creatures (moves them into the board)
    board.attacker.creatures.push(att);
    board.defender.creatures.push(def);

    let dmg = resolve_attack_and_apply_effects(0, 0, &atk_card, &mut board);
    assert_eq!(dmg, 20, "Lavalanche should deal 10 (base) + 5 (Fire) + 5 (Earth) = 20 total damage.");
}

/// Viperlash
/// base 15, Fire 0, Air 0, Earth 0, Water 0
/// No Special Effect.
#[test]
fn viperlash_test() {
    let mut board = board();

    let atk_card: AttackCard = load_single("data/cards/attacks/viperlash.json");
    let att: CreatureInstance = CreatureInstance::from(&heptadd());
    let def: CreatureInstance = CreatureInstance::from(&maxxor());

    // push creatures (moves them into the board)
    board.attacker.creatures.push(att);
    board.defender.creatures.push(def);

    let dmg = resolve_attack_and_apply_effects(0, 0, &atk_card, &mut board);
    assert_eq!(dmg, 15, "Viperlash should deal 15 (base) total damage.");
}
