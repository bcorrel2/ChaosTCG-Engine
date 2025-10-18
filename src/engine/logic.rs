use crate::engine::cards::*;
use std::collections::HashSet;

/* ---------- Runtime structs ---------- */

#[derive(Debug, Clone)]
pub struct CreatureInstance {
    pub card: CreatureCard,
    pub current_energy: i32,
    pub elements: HashSet<Element>, // start from card.elements, modified by gear/location if needed
    pub power: i32,
    pub wisdom: i32,
    pub courage: i32,
    pub speed: i32,
    pub mugic: i32,
}

impl From<&CreatureCard> for CreatureInstance {
    fn from(c: &CreatureCard) -> Self {
        let mut elems = HashSet::new();
        for e in &c.elements { elems.insert(e.clone()); }
        Self {
            card: c.clone(),
            current_energy: c.base_stats.energy as i32,
            elements: elems,
            power: c.base_stats.power as i32,
            wisdom: c.base_stats.wisdom as i32,
            courage: c.base_stats.courage as i32,
            speed: c.base_stats.speed as i32,
            mugic: c.mugic_counters as i32,
        }
    }
}

/* ---------- Simple helpers ---------- */

fn cond_passes(cre: &CreatureInstance, cond: &Condition) -> bool {
    match cond {
        Condition::Stat { stat, operator, value } => {
            let lhs = match stat.as_str() {
                "courage" => cre.courage,
                "power"   => cre.power,
                "wisdom"  => cre.wisdom,
                "speed"   => cre.speed,
                "energy"  => cre.current_energy,
                _ => return false,
            };
            match operator.as_str() {
                ">=" => lhs >= *value as i32,
                ">"  => lhs >  *value as i32,
                "==" => lhs == *value as i32,
                "<=" => lhs <= *value as i32,
                "<"  => lhs <  *value as i32,
                _    => false
            }
        }
        Condition::Tribe { tribe } => &cre.card.tribe == tribe,
    }
}

fn apply_stat_mods(cre: &mut CreatureInstance, mods: &StatModifiers) {
    if let Some(v) = mods.power   { cre.power   += v as i32; }
    if let Some(v) = mods.wisdom  { cre.wisdom  += v as i32; }
    if let Some(v) = mods.courage { cre.courage += v as i32; }
    if let Some(v) = mods.speed   { cre.speed   += v as i32; }
    if let Some(v) = mods.energy  { cre.current_energy += v as i32; } // treat +Energy as max & current for now
}

/* ---------- Public API for the test ---------- */

pub fn equip_battlegear(cre: &mut CreatureInstance, gear: &BattleGearCard) {
    for node in &gear.effects {
        match node.node_type.as_str() {
            "Continuous" => {
                if let Some(mods) = &node.modifiers {
                    apply_stat_mods(cre, mods);
                }
                if let Some(grant) = &node.grant {
                    cre.elements.insert(grant.element.clone());
                    // (If you later use element “bonus” like Fire 5 offensively, store it here.)
                }
            }
            "Conditional" => {
                if let Some(cond) = &node.condition {
                    if cond_passes(cre, cond) {
                        if let Some(mods) = &node.modifiers {
                            apply_stat_mods(cre, mods);
                        }
                        if let Some(grant) = &node.grant {
                            cre.elements.insert(grant.element.clone());
                        }
                    }
                }
            }
            _ => {}
        }
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

pub fn compute_attack_damage(attacker: &CreatureInstance, _defender: &CreatureInstance, atk: &AttackCard) -> i32 {
    let mut dmg = atk.damage.base as i32;
    // elemental adds if attacker has the element
    if attacker.elements.contains(&Element::Fire)  { dmg += atk.damage.fire  as i32; }
    if attacker.elements.contains(&Element::Air)   { dmg += atk.damage.air   as i32; }
    if attacker.elements.contains(&Element::Earth) { dmg += atk.damage.earth as i32; }
    if attacker.elements.contains(&Element::Water) { dmg += atk.damage.water as i32; }

    // If the attack has triggered/stat checks, you can evaluate here later.
    // For today, Pebblestorm has none, so we’re done.
    dmg.max(0)
}

pub fn deal_damage(defender: &mut CreatureInstance, amount: i32) {
    defender.current_energy = (defender.current_energy - amount).max(0);
}