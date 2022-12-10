#![allow(dead_code)]

use modular_bitfield::bitfield;
use modular_bitfield::specifiers::*;
use crate::character::Character;
use crate::d2s::stats::StatsBlock;

#[bitfield(filled = false)]
#[derive(BitfieldSpecifier)]
pub struct UnusedPoints {
    unused_stats_id: B9,
    unused_stats_value: B10,
    unused_skills_id: B9,
    unused_skills_value: B8
}

impl StatsBlock<UnusedPoints> for UnusedPoints {

    fn build(character: &Character) -> UnusedPoints {
        UnusedPoints::new()
            .with_unused_stats_id(4)
            .with_unused_stats_value(character.stat_points())
            .with_unused_skills_id(5)
            .with_unused_skills_value(character.skill_points())
    }
}