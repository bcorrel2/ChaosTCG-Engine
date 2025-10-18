use serde::{Deserialize, Serialize};

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
    MArrillians,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum Element {
    Fire,
    Air,
    Earth,
    Water,
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
    pub unique: bool,
    #[serde(default)]
    pub loyal: bool,

    pub rarity: String,
    pub set: CardSet,
    pub artist: String,

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
    Stat {
        stat: String,        // "courage", "power", ...
        operator: String,    // ">=", ">", "==", etc.
        value: i16,
    },
    Tribe {
        tribe: Tribe,
    },
}

/* ------------------------------ Triggers etc. ---------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trigger {
    pub event: String,                    // e.g., "AttackPlayed"
    pub source: String,                   // "Creature" | "Self"
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
    #[serde(default)]
    pub amount: Option<i16>,          // for Heal/Damage numeric amounts
    #[serde(default)]
    pub scope: Option<String>,        // e.g., "TargetEffect" for counters
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum EffectKind {
    Heal,
    Damage,
    Buff,
    Debuff,
    Counter,
    GrantAbility,
    SwitchBattlegear,
    Move,
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
/// without forcing you to rebuild the schema as you add cards.
///
/// This structure is intentionally permissive and matches all examples
/// you've provided (Ring of Na'arin, Whepcrack, Kiru City, UnderWorld City,
/// Decrescendo, Fortissimo, Flame Orb).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectNode {
    pub id: String,
    pub label: String,
    #[serde(rename = "type")]
    pub node_type: String, // "Continuous" | "Conditional" | "Triggered" | "Instant" | "TemporaryBuff" etc.

    #[serde(default)]
    pub target: Option<Target>,

    // For Continuous/Conditional modifiers:
    #[serde(default)]
    pub modifiers: Option<StatModifiers>,

    #[serde(default)]
    pub condition: Option<Condition>,

    // For Triggered hooks:
    #[serde(default)]
    pub trigger: Option<Trigger>,

    // For Location's "Challenge" or Attack's "Stat Check":
    #[serde(default)]
    pub challenge: Option<Challenge>,

    // For Effects That Require Comparing a Stat to a Sccuess Threshold
    #[serde(default)]
    pub stat_check: Option<StatCheck>,

    // For Attack's or Mugic's immediate payload (Decrescendo pattern)
    #[serde(default)]
    pub effect: Option<Effect>,

    // For BattleGear-style "Fire 5" grants:
    #[serde(default)]
    pub grant: Option<ElementGrant>,

    // For temporary buffs (Fortissimo pattern):
    #[serde(default)]
    pub duration: Option<Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Duration {
    EndOfTurn,
}