use crate::engine::cards::*;
use std::collections::{HashMap, HashSet};
use serde_json::Value;

/* ---------- Runtime structs ---------- */

#[derive(Debug, Clone)]
pub struct CreatureInstance {
    pub card: CreatureCard,
    pub current_energy: i32,
    pub elements: HashSet<Element>, // start from card.elements, modified by gear/location if needed
    pub elemental_bonus: HashMap<Element, i32>,
    pub power: i32,
    pub wisdom: i32,
    pub courage: i32,
    pub speed: i32,
    pub mugic: i32,
    pub equipped_gear: Option<BattleGearCard>,
    pub attacks_this_combat: u8,
}

impl From<&CreatureCard> for CreatureInstance {
    fn from(c: &CreatureCard) -> Self {
        let mut elems = HashSet::new();
        for e in &c.elements { elems.insert(e.clone()); }
        Self {
            card: c.clone(),
            current_energy: c.base_stats.energy as i32,
            elements: elems,
            elemental_bonus: HashMap::new(),
            power: c.base_stats.power as i32,
            wisdom: c.base_stats.wisdom as i32,
            courage: c.base_stats.courage as i32,
            speed: c.base_stats.speed as i32,
            mugic: c.mugic_counters as i32,
            equipped_gear: None,
            attacks_this_combat: 0
        }
    }
}

/* ---------- Simple helpers ---------- */

fn parse_element(s: &str) -> Option<Element> {
    match s.to_ascii_lowercase().as_str() {
        "fire"  => Some(Element::Fire),
        "air"   => Some(Element::Air),
        "earth" => Some(Element::Earth),
        "water" => Some(Element::Water),
        _ => None,
    }
}

fn tribe_eq(cre: &CreatureInstance, rhs: &str) -> bool {
    rhs.eq_ignore_ascii_case(&cre.card.tribe.to_string())
}

/// Compare a left and right JSON value using a string operator.
/// Supports strings, numbers (as i64), and bools. Unknown types -> false.
fn compare_json(lhs: &Value, op: &str, rhs: &Value) -> bool {
    match (lhs, rhs) {
        (Value::String(a), Value::String(b)) => match op {
            "==" => a.eq_ignore_ascii_case(b),
            "!=" => !a.eq_ignore_ascii_case(b),
            _ => false,
        },
        (Value::Bool(a), Value::Bool(b)) => match op {
            "==" => a == b,
            "!=" => a != b,
            _ => false,
        },
        // Treat numbers as i64; adjust to f64 if you need fractions later
        (Value::Number(a), Value::Number(b)) => {
            let (Some(la), Some(rb)) = (a.as_i64(), b.as_i64()) else { return false; };
            match op {
                "==" => la == rb,
                "!=" => la != rb,
                ">"  => la >  rb,
                ">=" => la >= rb,
                "<"  => la <  rb,
                "<=" => la <= rb,
                _    => false,
            }
        }
        _ => false,
    }
}

pub fn cond_passes(cre: &CreatureInstance, cond: &Condition) -> bool {
    match cond {
        Condition::All { all } => all.iter().all(|c| cond_passes(cre, c)),
        Condition::Any { any } => any.iter().any(|c| cond_passes(cre, c)),

        Condition::Compare { stat, operator, value } => {
            let stat_lc = stat.to_ascii_lowercase();

            // Map engine state to a JSON value we can compare with `compare_json`.
            // Handle common custom stats first.
            match stat_lc.as_str() {
                // --- Numbers (disciplines/energy) ---
                "courage" => return compare_json(&Value::from(cre.courage), operator, value),
                "power"   => return compare_json(&Value::from(cre.power),   operator, value),
                "wisdom"  => return compare_json(&Value::from(cre.wisdom),  operator, value),
                "speed"   => return compare_json(&Value::from(cre.speed),   operator, value),
                "energy"  => return compare_json(&Value::from(cre.current_energy), operator, value),

                // --- Name / Tribe as strings ---
                "name" => {
                    // If you don't store the name on the instance, use `cre.card.name.clone()`
                    let my_name = Value::from(cre.card.name.clone());
                    return compare_json(&my_name, operator, value);
                }
                "tribe" => {
                    if let Some(rhs) = value.as_str() {
                        // Only support equality/inequality for string tribe compare
                        return match operator.as_str() {
                            "==" => tribe_eq(cre, rhs),
                            "!=" => !tribe_eq(cre, rhs),
                            _    => false,
                        };
                    }
                    return false;
                }

                // --- Element membership: {"stat":"Element","operator":"==","value":"Air"} ---
                "element" => {
                    if let Some(elem_str) = value.as_str() {
                        if let Some(elem) = parse_element(elem_str) {
                            // Equality means "has this element", inequality means "does not have"
                            return match operator.as_str() {
                                "==" => cre.elements.contains(&elem),
                                "!=" => !cre.elements.contains(&elem),
                                _    => false,
                            };
                        }
                    }
                    return false;
                }
                // Fallback: unknown stat
                _ => false,
            }
        }
    }
}

fn apply_stat_mods(cre: &mut CreatureInstance, mods: &StatModifiers) {
    if let Some(v) = mods.power   { cre.power   += v as i32; }
    if let Some(v) = mods.wisdom  { cre.wisdom  += v as i32; }
    if let Some(v) = mods.courage { cre.courage += v as i32; }
    if let Some(v) = mods.speed   { cre.speed   += v as i32; }
    if let Some(v) = mods.energy  { cre.current_energy += v as i32; } // treat +Energy as max & current for now
}

fn stat_value(cre: &CreatureInstance, disc: &str) -> i32 {
    match disc {
        "Power" => cre.power,
        "Courage" => cre.courage,
        "Wisdom" => cre.wisdom,
        "Speed" => cre.speed,
        "Energy" => cre.current_energy,
        _ => 0,
    }
}

/* ---------- Public API for the test ---------- */

// src/engine/logic.rs

pub fn equip_battlegear(cre: &mut CreatureInstance, gear: &BattleGearCard) {
    cre.equipped_gear = Some(gear.clone());
    apply_battlegear_effects(cre, gear);
}

// src/engine/logic.rs

fn apply_battlegear_effects(cre: &mut CreatureInstance, gear: &BattleGearCard) {
    for node in &gear.effects {
        let kind = node.node_type.to_ascii_lowercase();

        match kind.as_str() {
            // Always-on effects
            "continuous" => {
                if let Some(eff) = &node.effect {
                    let _ = apply_effect_to_creature(cre, eff);
                }
                if let Some(gr) = &node.grant {
                    apply_element_grant(cre, gr);
                }
            }

            // Conditional effects (e.g., Whepcrack: UnderWorld → Fire 5)
            "conditional" => {
                let cond_ok = node.condition.as_ref().map_or(false, |c| cond_passes(cre, c));
                if !cond_ok { continue; }

                if let Some(eff) = &node.effect {
                    let _ = apply_effect_to_creature(cre, eff);
                }
                if let Some(gr) = &node.grant {
                    apply_element_grant(cre, gr);
                }
            }

            _ => { /* ignore other node types here */ }
        }
    }
}

fn apply_element_grant(cre: &mut CreatureInstance, grant: &ElementGrant) {
    let elem = grant.element.clone();
    cre.elements.insert(elem.clone());
    *cre.elemental_bonus.entry(elem).or_insert(0) += i32::from(grant.bonus);
}

use crate::engine::cards::{Effect, EffectKind};

/// Apply a single effect to a creature's live stats.
/// Returns true if anything was applied.
fn apply_effect_to_creature(cre: &mut CreatureInstance, eff: &Effect) -> bool {
    match eff.kind {
        EffectKind::ModifyDiscipline => {
            // amount is i16 in JSON; convert safely
            let amt: i32 = eff.amount.unwrap_or(0).into();
            // be forgiving about case in JSON: "Energy", "energy", etc.
            let d = eff.discipline.as_deref().unwrap_or("").to_ascii_lowercase();
            match d.as_str() {
                "courage" => { cre.courage += amt; true }
                "power"   => { cre.power   += amt; true }
                "wisdom"  => { cre.wisdom  += amt; true }
                "speed"   => { cre.speed   += amt; true }
                "energy"  => {
                    cre.current_energy += amt;
                    if cre.current_energy < 0 { cre.current_energy = 0; }
                    true
                }
                _ => false, // unknown discipline; ignore
            }
        }
        _ => false, // not handled here (fine for now)
    }
}

pub fn apply_location(creatures: &mut [&mut CreatureInstance], loc: &LocationCard) {
    for node in &loc.effects {
        if node.node_type.as_str() == "Continuous" {
            for c in creatures.iter_mut() {
                // c: &mut &mut CreatureInstance  →  cre: &mut CreatureInstance
                let cre: &mut CreatureInstance = &mut **c;

                let passes = if let Some(cond) = &node.condition {
                    cond_passes(cre, cond)
                } else {
                    true
                };

                if passes {
                    if let Some(mods) = &node.modifiers {
                        apply_stat_mods(cre, mods);
                    }
                }
            }
        }
    }
}

fn is_attack_event(s: &str) -> bool {
    let e = s.to_ascii_lowercase();
    e == "attackplayed" || e == "attack" || e == "onattack"
}

pub fn add_location_attack_bonus(
    attacker: &CreatureInstance,
    defender: &CreatureInstance,
    loc: &LocationCard,
) -> i32 {
    use crate::engine::cards::EffectKind;
    let mut extra = 0;

    for node in &loc.effects {
        if !node.node_type.eq_ignore_ascii_case("Triggered") { continue; }
        let trg = match &node.trigger { Some(t) => t, None => continue };
        if !is_attack_event(&trg.event) { continue; }

        // (If you wired first-attack flags, handle them here; otherwise ignore.)

        // Condition (e.g., Tribe == UnderWorlders)
        if let Some(cond) = &node.condition {
            if !cond_passes(attacker, cond) { continue; }
        }

        // Direct damage/modify-damage on the node
        if let Some(eff) = &node.effect {
            if eff.kind == EffectKind::ModifyDamage || eff.kind == EffectKind::Damage {
                if let Some(a) = eff.amount { extra += i32::from(a); }
            }
        }

        // Challenge (e.g., Power 15: Deal 5)
        if let Some(ch) = &node.challenge {
            let val = stat_value(attacker, &ch.discipline); // includes gear mods
            if val >= ch.threshold as i32 {
                let se = &ch.success_effect;
                if (se.kind == EffectKind::ModifyDamage || se.kind == EffectKind::Damage) && se.amount.is_some() {
                    extra += i32::from(se.amount.unwrap());
                }
            }
        }
    }

    extra
}

pub fn compute_attack_damage(attacker: &CreatureInstance, _defender: &CreatureInstance, atk: &AttackCard) -> i32 {
    let mut dmg = atk.damage.base as i32;

    if attacker.elements.contains(&Element::Fire) {
        let bonus = *attacker.elemental_bonus.get(&Element::Fire).unwrap_or(&0);
        dmg += atk.damage.fire as i32 + bonus;
    }
    if attacker.elements.contains(&Element::Air) {
        let bonus = *attacker.elemental_bonus.get(&Element::Air).unwrap_or(&0);
        dmg += atk.damage.air as i32 + bonus;
    }
    if attacker.elements.contains(&Element::Earth) {
        let bonus = *attacker.elemental_bonus.get(&Element::Earth).unwrap_or(&0);
        dmg += atk.damage.earth as i32 + bonus;
    }
    if attacker.elements.contains(&Element::Water) {
        let bonus = *attacker.elemental_bonus.get(&Element::Water).unwrap_or(&0);
        dmg += atk.damage.water as i32 + bonus;
    }

    // Stat checks (i.e. Flame Orb "Power 75: +10")
    for node in &atk.effects {
        if let Some(sc) = &node.challenge {
            let val = stat_value(attacker, &sc.discipline);
            if val >= sc.threshold as i32 {
                if sc.success_effect.kind == EffectKind::Damage {
                    if let Some(extra) = sc.success_effect.amount {
                        dmg += i32::from(extra);
                    }
                }
            }
        }
    }

    dmg.max(0)
}

pub fn deal_damage(defender: &mut CreatureInstance, amount: i32) {
    defender.current_energy = (defender.current_energy - amount).max(0);
}
