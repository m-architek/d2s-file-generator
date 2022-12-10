#![allow(dead_code)]
#![allow(unused_braces)]

use modular_bitfield::bitfield;
use modular_bitfield::specifiers::*;

use crate::character::Character;

#[bitfield(filled = false)]
#[derive(BitfieldSpecifier)]
pub struct Experience {
    experience_id: B9,
    experience_value: B32
}

impl Experience {

    pub fn build(character: &Character) -> Experience {
        Experience::new()
            .with_experience_id(13)
            .with_experience_value(character.experience())
    }
}