#![allow(dead_code)]

use modular_bitfield::bitfield;
use modular_bitfield::specifiers::*;
use crate::character::Character;
use crate::d2s::stats::StatsBlock;

#[bitfield(filled = false)]
#[derive(BitfieldSpecifier)]
pub struct Stamina {
    stamina_current_id: B9,
    stamina_current_value: B21,
    stamina_max_id: B9,
    stamina_max_value: B21
}

impl StatsBlock<Stamina> for Stamina {

    fn build(character: &Character) -> Stamina {
        Stamina::new()
            .with_stamina_current_id(10)
            .with_stamina_current_value(character.stamina() * 256)
            .with_stamina_max_id(11)
            .with_stamina_max_value(character.stamina() * 256)
    }
}