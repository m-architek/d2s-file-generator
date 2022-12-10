#![allow(dead_code)]
#![allow(unused_braces)]

use modular_bitfield::bitfield;
use modular_bitfield::specifiers::*;
use crate::character::Character;

#[bitfield(filled = false)]
#[derive(BitfieldSpecifier)]
pub struct Life {
    hp_current_id: B9,
    hp_current_value: B21,
    hp_max_id: B9,
    hp_max_value: B21
}

impl Life {

    pub fn build(character: &Character) -> Life {
        Life::new()
            .with_hp_current_id(6)
            .with_hp_current_value(character.hp() * 256)
            .with_hp_max_id(7)
            .with_hp_max_value(character.hp() * 256)
    }
}