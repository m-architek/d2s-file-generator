#![allow(dead_code)]

use modular_bitfield::bitfield;
use modular_bitfield::specifiers::*;
use crate::character::Character;
use crate::d2s::stats::StatsBlock;

#[bitfield(filled = false)]
#[derive(BitfieldSpecifier)]
pub struct Mana {
    mana_current_id: B9,
    mana_current_value: B21,
    mana_max_id: B9,
    mana_max_value: B21
}

impl StatsBlock<Mana> for Mana {

    fn build(character: &Character) -> Mana {
        Mana::new()
            .with_mana_current_id(8)
            .with_mana_current_value(character.mana() * 256)
            .with_mana_max_id(9)
            .with_mana_max_value(character.mana() * 256)
    }
}