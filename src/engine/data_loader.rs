use std::fs;
use std::path::Path;

use serde::de::DeserializeOwned;

use crate::engine::cards::{
    AttackCard, BattleGearCard, CreatureCard, LocationCard, MugicCard,
};

/// Generic helper to load all JSON files in a directory into a Vec<T>.
/// Works for any type that implements DeserializeOwned (all your card structs do).
pub fn load_dir<T, P>(dir: P) -> Vec<T>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let mut cards = Vec::new();
    let dir_path = dir.as_ref();

    for entry in fs::read_dir(dir_path).expect("Failed to read directory") {
        let path = entry.expect("Failed to read entry").path();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            match fs::read_to_string(&path) {
                Ok(data) => match serde_json::from_str::<T>(&data) {
                    Ok(card) => cards.push(card),
                    Err(e) => eprintln!("❌ Parse error in {:?}: {}", path, e),
                },
                Err(e) => eprintln!("❌ Failed to read {:?}: {}", path, e),
            }
        }
    }

    cards
}

/// Convenience wrappers for each card type directory
pub fn load_creatures<P: AsRef<Path>>(dir: P) -> Vec<CreatureCard> {
    load_dir(dir)
}

pub fn load_attacks<P: AsRef<Path>>(dir: P) -> Vec<AttackCard> {
    load_dir(dir)
}

pub fn load_battlegear<P: AsRef<Path>>(dir: P) -> Vec<BattleGearCard> {
    load_dir(dir)
}

pub fn load_mugic<P: AsRef<Path>>(dir: P) -> Vec<MugicCard> {
    load_dir(dir)
}

pub fn load_locations<P: AsRef<Path>>(dir: P) -> Vec<LocationCard> {
    load_dir(dir)
}

/// Utility: load a single JSON file directly
pub fn load_single<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> T {
    let data = fs::read_to_string(path).expect("Failed to read file");
    serde_json::from_str::<T>(&data).expect("Failed to parse JSON")
}