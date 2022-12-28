#![allow(dead_code)]
#![allow(unused_braces)]

use modular_bitfield::bitfield;
use modular_bitfield::specifiers::*;

use crate::character::Character;

#[bitfield(filled = false)]
#[derive(BitfieldSpecifier)]
pub struct Gold {
    stashed_gold_id: B9,
    stashed_gold_value: B25
}

impl Gold {

    pub fn build(character: &Character) -> Gold {
        Gold::new()
            .with_stashed_gold_id(15)
            .with_stashed_gold_value(*character.gold)
    }
}