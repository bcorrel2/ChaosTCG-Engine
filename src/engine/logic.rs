use crate::engine::cards::*;
use std::collections::{HashMap, HashSet};

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

pub fn equip_battlegear(cre: &mut CreatureInstance, gear: &BattleGearCard) {
    for node in &gear.effects {
        match node.node_type.as_str() {
            "Continuous" => {
                if let Some(mods) = &node.modifiers {
                    apply_stat_mods(cre, mods);
                }
                if let Some(grant) = &node.grant {
                    cre.elements.insert(grant.element);
                    if grant.bonus != 0 {
                        *cre.elemental_bonus.entry(grant.element).or_insert(0) += i32::from(grant.bonus);
                    }
                }
            }
            "Conditional" => {
                if let Some(cond) = &node.condition {
                    if cond_passes(cre, cond) {
                        if let Some(mods) = &node.modifiers {
                            apply_stat_mods(cre, mods);
                        }
                        if let Some(grant) = &node.grant {
                            cre.elements.insert(grant.element);
                            if grant.bonus != 0 {
                                *cre.elemental_bonus.entry(grant.element).or_insert(0) += i32::from(grant.bonus);
                            }
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

pub fn add_location_attack_bonus(attacker: &CreatureInstance, _defender: &CreatureInstance, loc: &LocationCard) -> i32 {
    let mut extra = 0;
    for node in &loc.effects {
        if node.node_type == "Triggered" {
            if let Some(trg) = &node.trigger {
                if trg.event == "AttackPlayed" {
                    // Check optional tribe condition
                    let cond_ok = if let Some(cond) = &trg.condition {
                        cond_passes(attacker, cond)
                    } else { true };

                    if cond_ok {
                        if let Some(ch) = &node.challenge {
                            let val = stat_value(attacker, &ch.discipline);
                            if val >= ch.threshold as i32 {
                                if ch.success_effect.kind == EffectKind::Damage {
                                    if let Some(a) = ch.success_effect.amount {
                                        extra += i32::from(a); // ← cast i16 → i32
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    extra as i32
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
        if let Some(sc) = &node.stat_check {
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
