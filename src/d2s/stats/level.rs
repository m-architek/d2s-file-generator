#![allow(dead_code)]
#![allow(unused_braces)]

use modular_bitfield::bitfield;
use modular_bitfield::specifiers::*;

use crate::character::Character;

#[bitfield]
#[derive(BitfieldSpecifier)]
pub struct Level {
    level_id: B9,
    level_value: B7
}

impl Level {

    pub fn build(character: &Character) -> Level {
        Level::new()
            .with_level_id(12)
            .with_level_value(*character.level)
    }
}