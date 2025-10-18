use serde::{Deserialize, Serialize};
use std::fmt;

/// Top-level card enum can deserialize mixed lists (e.g., a single bundle).
/// This uses the `"type"` field in the JSON as an internal tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Card {
    Creature(CreatureCard),
    Attack(AttackCard),
    BattleGear(BattleGearCard),
    Mugic(MugicCard),
    Location(LocationCard),
}

/* ----------------------------- Shared Basics ----------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardSet {
    pub name: String,
    pub number: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum Tribe {
    OverWorlders,
    UnderWorlders,
    Mipedians,
    Danians,
    #[serde(rename = "M'arrillians")]
    Marrillians,
}

impl fmt::Display for Tribe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Tribe::OverWorlders => "OverWorlders",
            Tribe::UnderWorlders => "UnderWorlders",
            Tribe::Mipedians => "Mipedians",
            Tribe::Danians => "Danians",
            Tribe::Marrillians => "M'arrillians",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum Element {
    Fire,
    Air,
    Earth,
    Water,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::Fire  => write!(f, "Fire"),
            Element::Water => write!(f, "Water"),
            Element::Air   => write!(f, "Air"),
            Element::Earth => write!(f, "Earth"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub courage: u16,
    pub power: u16,
    pub wisdom: u16,
    pub speed: u16,
    pub energy: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsRange {
    pub courage: [u16; 2],
    pub power: [u16; 2],
    pub wisdom: [u16; 2],
    pub speed: [u16; 2],
    pub energy: [u16; 2],
}

/* ------------------------------ CreatureCard ----------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatureCard {
    pub id: String,
    pub name: String,
    pub tribe: Tribe,
    #[serde(default)]
    pub subtypes: Vec<String>,
    #[serde(default)]
    pub loyal: bool,

    pub rarity: String,
    pub set: CardSet,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,

    #[serde(default)]
    pub unique: bool,

    pub base_stats: Stats,
    pub stat_ranges: StatsRange,

    pub mugic_counters: u8,
    #[serde(default)]
    pub elements: Vec<Element>,

    pub rules_text: String,
    #[serde(default)]
    pub abilities: Vec<Ability>,

    #[serde(default)]
    pub flavor_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub id: String,
    pub label: String,
    #[serde(rename = "type")]
    pub ability_type: AbilityType,
    #[serde(default)]
    pub cost: Option<AbilityCost>,
    #[serde(default)]
    pub timing: Option<String>,
    #[serde(default)]
    pub target: Option<Target>,
    pub effect: Effect, // simple core effect object (e.g., Heal 10)
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AbilityType {
    Activated,
    Triggered,
    Innate,
    Continuous,
    Conditional
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AbilityCost {
    #[serde(default)]
    pub mugic: Option<u8>,
}

/* --------------------------------- Attack -------------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackCard {
    pub id: String,
    pub name: String,
    pub rarity: String,
    pub set: CardSet,
    pub build_points: u8,
    pub damage: AttackDamage,
    #[serde(default)]
    pub rules_text: Option<String>,
    #[serde(default)]
    pub effects: Vec<EffectNode>,
    #[serde(default)]
    pub flavor_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackDamage {
    pub base: u8,
    pub fire: u8,
    pub air: u8,
    pub earth: u8,
    pub water: u8,
}

/* ------------------------------- BattleGear ------------------------------ */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleGearCard {
    pub id: String,
    pub name: String,
    pub rarity: String,
    pub set: CardSet,
    pub rules_text: String,
    pub effects: Vec<EffectNode>,
    #[serde(default)]
    pub flavor_text: Option<String>,
}

/* --------------------------------- Mugic --------------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MugicCard {
    pub id: String,
    pub name: String,
    pub mugic_type: MugicType, // "Generic" etc.
    pub rarity: String,
    pub set: CardSet,
    pub mugic_cost: u8,
    pub rules_text: String,
    #[serde(default)]
    pub effects: Vec<EffectNode>,
    #[serde(default)]
    pub flavor_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MugicType {
    Generic,
    // Add tribe-specific types later if you decide to encode them
}

/* -------------------------------- Location ------------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationCard {
    pub id: String,
    pub name: String,
    pub rarity: String,
    pub set: CardSet,
    pub rules_text: String,
    pub effects: Vec<EffectNode>,
    #[serde(default)]
    pub flavor_text: Option<String>,
}

/* --------------------------- Targets / Conditions ------------------------ */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    #[serde(rename = "type")]
    pub target_type: String, // e.g., "Creature", "OpponentCreature", "EquippedCreature", "Effect"
    pub controller: String,  // "self" | "ally" | "opponent" | "any"
    // You can extend with more filters later
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatModifiers {
    #[serde(default)] pub courage: Option<i16>,
    #[serde(default)] pub power:   Option<i16>,
    #[serde(default)] pub wisdom:  Option<i16>,
    #[serde(default)] pub speed:   Option<i16>,
    #[serde(default)] pub energy:  Option<i16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Condition {
    /// Generic comparator:
    /// {"stat":"Tribe","operator":"==","value":"Mipedians"}
    /// {"stat":"Engaged","operator":"==","value":true}
    /// {"stat":"Power","operator":">=","value":75}
    Compare {
        stat: String,
        operator: String,
        #[serde(default)]
        value: serde_json::Value, // allows string/number/bool/null
    },

    /// Conjunction of conditions: {"all":[ {...}, {...} ]}
    All {
        all: Vec<Condition>,
    },

    /// Disjunction of conditions: {"any":[ {...}, {...} ]}
    Any {
        any: Vec<Condition>,
    },
}

/* ------------------------------ Triggers etc. ---------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trigger {
    pub event: String,                          // e.g. "AttackPlayed", "CombatStart", "BeforeMugicCast"

    // Make everything else optional/with defaults so older JSON parses cleanly.
    #[serde(default)]
    pub source: Option<String>,                 // e.g. "Attacker", "Defender", "Self", "Any"

    #[serde(default)]
    pub first_attack_this_combat: bool,         // used by Mipedim Oasis

    // Add other flags you might have referenced elsewhere; keep them optional:
    #[serde(default)]
    pub at_location_activation: bool,           // if you used similar flags
    #[serde(default)]
    pub when_revealed: bool,                // "Creature" | "Self"

    #[serde(default)]
    pub condition: Option<Condition>,     // Optional extra filter
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Challenge {
    pub discipline: String,   // "Power", "Courage", etc.
    pub threshold: u16,
    pub success_effect: Effect,
}

/* ---------------------------- Effect Primitives -------------------------- */

/// Core atomic effect used in a few places (e.g., Ability.effect, Challenge.success_effect).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub kind: EffectKind,
    #[serde(default)] pub amount: Option<i16>,
    #[serde(default)] pub discipline: Option<String>,
    #[serde(default)] pub duration: Option<String>,
    #[serde(default)] pub elements: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum EffectKind {
    Heal,
    Damage,                 // ← needed by logic.rs (Combat / stat checks)
    GrantAbility,
    SwitchBattlegear,
    Move,
    Counter,

    // Discipline & Damage modifiers
    ModifyDiscipline,       // e.g., Lightning Burst (-25 Power EOT), Gothos Tower
    ModifyDamage,           // e.g., Everrain, Gloomuck, Storm Tunnel
    SetAttackDamage,        // e.g., Crystal Cave (first attack deals 0)

    // Mugic / Counters / Keywords
    ModifyMugicCost,        // Stone/Glacier/Wooden Pillar
    AddMugicCounters,       // Castle Pillar, Gigantempopolis
    ActivateKeyword,        // Mount Pillar (Hive)

    // Elements / Properties
    GrantElements,          // Stronghold Morn
    GrantProperty,          // Lake Ken-I-Po (Untargetable)

    // Cards/z=Zones and Play Restrictions
    ReturnCard,             // Castle Bodhran (discard → hand)
    RevealCards,            // Windslash (flip gear)
    RestrictPlay,           // Runic Grove (only Generic Mugic)
    DisableActions,         // Dranakis Threshold (no Mugic/activated)

    // Deck manipulation
    DeckManipulation,       // Ravanaugh Ridge (scry 3)

    // Movement mode (from gear)
    GainMovementMode,       // Mipedian Cactus (teleport)

    // Scaling energy (Danian)
    ModifyEnergyPerCondition,

    // Pre-combat strike
    PreemptiveDamage,
}

/// Used by some BattleGear effects like "Fire 5".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementGrant {
    pub element: Element,
    pub bonus: i16,
}

/* --------------------------  Stat Checks  ---------------------------------*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatCheck {
    pub discipline: String,   // "Power", "Wisdom", etc.
    pub threshold: u16,
    pub success_effect: Effect,
}

/* ---------------------------- EffectNode (rich) -------------------------- */

/// A flexible "effect node" that can represent:
/// - continuous stat buffs,
/// - conditional effects,
/// - triggered effects with challenges,
/// - instant damage,
/// - temporary buffs (via duration),

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectNode {
    pub id: String,
    pub label: String,
    #[serde(rename = "type")]
    pub node_type: String,

    #[serde(default)] pub trigger: Option<Trigger>,
    #[serde(default)] pub condition: Option<Condition>,
    #[serde(default)] pub target: Option<Target>,
    #[serde(default)] pub effect: Option<Effect>,
    #[serde(default)] pub grant: Option<ElementGrant>,
    #[serde(default)] pub challenge: Option<Challenge>,
    #[serde(default)] pub modifiers: Option<StatModifiers>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Duration {
    EndOfTurn,
}