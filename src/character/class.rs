use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Class {
    Barbarian,
    Paladin,
    Sorceress,
    Necromancer,
    Amazon,
    Druid,
    Assassin
}

impl Class {

    pub fn base_strength(&self) -> u16 {
        match self {
            Class::Barbarian => 30,
            Class::Paladin => 25,
            Class::Sorceress => 10,
            Class::Necromancer => 15,
            Class::Amazon => 20,
            Class::Druid => 15,
            Class::Assassin => 20
        }
    }

    pub fn base_dexterity(&self) -> u16 {
        match self {
            Class::Barbarian => 20,
            Class::Paladin => 20,
            Class::Sorceress => 25,
            Class::Necromancer => 25,
            Class::Amazon => 25,
            Class::Druid => 20,
            Class::Assassin => 20
        }
    }

    pub fn base_vitality(&self) -> u16 {
        match self {
            Class::Barbarian => 25,
            Class::Paladin => 25,
            Class::Sorceress => 10,
            Class::Necromancer => 15,
            Class::Amazon => 20,
            Class::Druid => 25,
            Class::Assassin => 20
        }
    }

    pub fn base_energy(&self) -> u16 {
        match self {
            Class::Barbarian => 10,
            Class::Paladin => 15,
            Class::Sorceress => 35,
            Class::Necromancer => 25,
            Class::Amazon => 15,
            Class::Druid => 20,
            Class::Assassin => 25
        }
    }

    pub fn base_hp(&self) -> f32 {
        match self {
            Class::Barbarian => 55.0,
            Class::Paladin => 55.0,
            Class::Sorceress => 40.0,
            Class::Necromancer => 45.0,
            Class::Amazon => 50.0,
            Class::Druid => 55.0,
            Class::Assassin => 50.0
        }
    }

    pub fn base_mana(&self) -> f32 {
        match self {
            Class::Barbarian => 10.0,
            Class::Paladin => 15.0,
            Class::Sorceress => 35.0,
            Class::Necromancer => 25.0,
            Class::Amazon => 15.0,
            Class::Druid => 20.0,
            Class::Assassin => 25.0
        }
    }

    pub fn base_stamina(&self) -> f32 {
        match self {
            Class::Barbarian => 92.0,
            Class::Paladin => 89.0,
            Class::Sorceress => 74.0,
            Class::Necromancer => 79.0,
            Class::Amazon => 84.0,
            Class::Druid => 84.0,
            Class::Assassin => 95.0
        }
    }

    pub fn level_up_hp(&self) -> f32 {
        match self {
            Class::Barbarian => 2.0,
            Class::Paladin => 2.0,
            Class::Sorceress => 1.0,
            Class::Necromancer => 1.5,
            Class::Amazon => 2.0,
            Class::Druid => 1.5,
            Class::Assassin => 2.0
        }
    }

    pub fn level_up_mana(&self) -> f32 {
        match self {
            Class::Barbarian => 1.0,
            Class::Paladin => 1.5,
            Class::Sorceress => 2.0,
            Class::Necromancer => 2.0,
            Class::Amazon => 1.5,
            Class::Druid => 2.0,
            Class::Assassin => 1.5
        }
    }

    pub fn level_up_stamina(&self) -> f32 {
        match self {
            Class::Barbarian => 1.0,
            Class::Paladin => 1.0,
            Class::Sorceress => 1.0,
            Class::Necromancer => 1.0,
            Class::Amazon => 1.0,
            Class::Druid => 1.0,
            Class::Assassin => 1.25
        }
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
