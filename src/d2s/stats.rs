#![allow(dead_code)]

use modular_bitfield::bitfield;
use modular_bitfield::specifiers::*;

use crate::character::Character;
use crate::d2s::stats::attributes::Attributes;
use crate::d2s::stats::experience::Experience;
use crate::d2s::stats::gold::Gold;
use crate::d2s::stats::level::Level;
use crate::d2s::stats::life::Life;
use crate::d2s::stats::mana::Mana;
use crate::d2s::stats::stamina::Stamina;
use crate::d2s::stats::unused_points::UnusedPoints;

mod attributes;
mod unused_points;
mod life;
mod mana;
mod stamina;
mod level;
mod experience;
mod gold;

const SECTION_TERMINATOR: u16 = 0x1ff;

pub fn build_stats(character: &Character) -> Vec<u8> {
    let mut stats: Vec<u8> = Vec::new();

    let header: [u8; 2] = [103, 102];
    let body: Vec<u8> = match character.level > 1 {
        true => match character.gold() > 0 {
            true => StatsWithExpWithGold::build(character).into_bytes().to_vec(),
            _ => StatsWithExp::build(character).into_bytes().to_vec()
        },
        _ => match character.gold() > 0 {
            true => StatsWithGold::build(character).into_bytes().to_vec(),
            _ => Stats::build(character).into_bytes().to_vec()
        }
    };

    stats.extend(&header);
    stats.extend(&body);
    stats
}

#[bitfield(filled = false)]
struct Stats {
    attributes: Attributes,
    life: Life,
    mana: Mana,
    stamina: Stamina,
    level: Level,
    section_terminator: B9
}

impl Stats {
    fn build(character: &Character) -> Stats {
        Stats::new()
            .with_attributes(Attributes::build(character))
            .with_life(Life::build(character))
            .with_mana(Mana::build(character))
            .with_stamina(Stamina::build(character))
            .with_level(Level::build(character))
            .with_section_terminator(SECTION_TERMINATOR)
    }
}

#[bitfield(filled = false)]
struct StatsWithGold {
    attributes: Attributes,
    life: Life,
    mana: Mana,
    stamina: Stamina,
    level: Level,
    gold: Gold,
    section_terminator: B9
}

impl StatsWithGold {
    fn build(character: &Character) -> StatsWithGold {
        StatsWithGold::new()
            .with_attributes(Attributes::build(character))
            .with_life(Life::build(character))
            .with_mana(Mana::build(character))
            .with_stamina(Stamina::build(character))
            .with_level(Level::build(character))
            .with_gold(Gold::build(character))
            .with_section_terminator(SECTION_TERMINATOR)
    }
}

#[bitfield(filled = false)]
struct StatsWithExp {
    attributes: Attributes,
    unused_points: UnusedPoints,
    life: Life,
    mana: Mana,
    stamina: Stamina,
    level: Level,
    experience: Experience,
    section_terminator: B9
}

impl StatsWithExp {
    fn build(character: &Character) -> StatsWithExp {
        StatsWithExp::new()
            .with_attributes(Attributes::build(character))
            .with_unused_points(UnusedPoints::build(character))
            .with_life(Life::build(character))
            .with_mana(Mana::build(character))
            .with_stamina(Stamina::build(character))
            .with_level(Level::build(character))
            .with_experience(Experience::build(character))
            .with_section_terminator(SECTION_TERMINATOR)
    }
}

#[bitfield]
struct StatsWithExpWithGold {
    attributes: Attributes,
    unused_points: UnusedPoints,
    life: Life,
    mana: Mana,
    stamina: Stamina,
    level: Level,
    experience: Experience,
    gold: Gold,
    section_terminator: B9
}

impl StatsWithExpWithGold {
    fn build(character: &Character) -> StatsWithExpWithGold {
        StatsWithExpWithGold::new()
            .with_attributes(Attributes::build(character))
            .with_unused_points(UnusedPoints::build(character))
            .with_life(Life::build(character))
            .with_mana(Mana::build(character))
            .with_stamina(Stamina::build(character))
            .with_level(Level::build(character))
            .with_experience(Experience::build(character))
            .with_gold(Gold::build(character))
            .with_section_terminator(SECTION_TERMINATOR)
    }
}
