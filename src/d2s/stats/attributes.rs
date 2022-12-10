#![allow(dead_code)]

use modular_bitfield::bitfield;
use modular_bitfield::specifiers::*;
use crate::character::Character;
use crate::d2s::stats::StatsBlock;

#[bitfield(filled = false)]
#[derive(BitfieldSpecifier)]
pub struct Attributes {
    strength_id: B9,
    strength_value: B10,
    energy_id: B9,
    energy_value: B10,
    dexterity_id: B9,
    dexterity_value: B10,
    vitality_id: B9,
    vitality_value: B10
}

impl StatsBlock<Attributes> for Attributes {

    fn build(character: &Character) -> Attributes {
        Attributes::new()
            .with_strength_id(0)
            .with_strength_value(character.strength())
            .with_energy_id(1)
            .with_energy_value(character.energy())
            .with_dexterity_id(2)
            .with_dexterity_value(character.dexterity())
            .with_vitality_id(3)
            .with_vitality_value(character.vitality())
    }
}
