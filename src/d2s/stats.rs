use modular_bitfield::bitfield;
use modular_bitfield::specifiers::*;

use crate::character::Character;

pub fn build_stats(character: &Character) -> Vec<u8> {
    let mut stats: Vec<u8> = Vec::new();

    let header: [u8; 2] = [103, 102];

    let body = Stats::new()
        .with_strength_id(0)
        .with_eof(0x1ff);

    stats.extend(&header);
    stats.extend(&body.into_bytes());
    stats
}

#[bitfield(filled = false)]
#[derive(Debug)]
struct Stats {
    strength_id: B9,
    strength_value: B10,
    energy_id: B9,
    energy_value: B10,
    dexterity_id: B9,
    dexterity_value: B10,
    vitality_id: B9,
    vitality_value: B10,
    unused_stats_id: B9,
    unused_stats_value: B10,
    unused_skills_id: B9,
    unused_skills_value: B8,
    hp_current_id: B9,
    hp_current_value: B21,
    hp_max_id: B9,
    hp_max_value: B21,
    mana_current_id: B9,
    mana_current_value: B21,
    mana_max_id: B9,
    mana_max_value: B21,
    stamina_current_id: B9,
    stamina_current_value: B21,
    stamina_max_id: B9,
    stamina_max_value: B21,
    level_id: B9,
    level_value: B7,
    experience_id: B9,
    experience_value: B32,
    gold_id: B9,
    gold_value: B25,
    stashed_gold_id: B9,
    stashed_gold_value: B25,
    eof: B9
}
