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

pub fn build_stats(character: &Character) -> Vec<u8> {
    let mut stats: Vec<u8> = Vec::new();

    let header: [u8; 2] = [103, 102];

    let body = Stats::new()
        .with_attributes(Attributes::build(character))
        .with_unused_points(UnusedPoints::build(character))
        .with_life(Life::build(character))
        .with_mana(Mana::build(character))
        .with_stamina(Stamina::build(character))
        .with_level(Level::build(character))
        .with_experience(Experience::build(character))
        .with_gold(Gold::build(character))
        .with_section_terminator(0x1ff);

    stats.extend(&header);
    stats.extend(&body.into_bytes());
    stats
}

trait StatsBlock<T> {
    fn build(character: &Character) -> T;
}

#[bitfield]
struct Stats {
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
