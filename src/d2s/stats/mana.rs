#![allow(dead_code)]
#![allow(unused_braces)]

use modular_bitfield::bitfield;
use modular_bitfield::specifiers::*;
use crate::character::Character;

#[bitfield(filled = false)]
#[derive(BitfieldSpecifier)]
pub struct Mana {
    mana_current_id: B9,
    mana_current_value: B21,
    mana_max_id: B9,
    mana_max_value: B21
}

impl Mana {

    pub fn build(character: &Character) -> Mana {
        Mana::new()
            .with_mana_current_id(8)
            .with_mana_current_value((character.mana() * 256.0) as u32)
            .with_mana_max_id(9)
            .with_mana_max_value((character.mana() * 256.0) as u32)
    }
}