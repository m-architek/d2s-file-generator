use class::Class;
use difficulty::Difficulty;
use mode::Mode;

pub mod class;
pub mod mode;
pub mod difficulty;
mod experience;

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub class: Class,
    pub level: u8,
    pub mode: Mode,
    pub completed_difficulty: Option<Difficulty>,
    pub last_played: u32,
    pub map_id: u32
}

impl Character {

    pub fn strength(&self) -> u16 {
        30
    }

    pub fn dexterity(&self) -> u16 {
        20
    }

    pub fn vitality(&self) -> u16 {
        25
    }

    pub fn energy(&self) -> u16 {
        10
    }

    pub fn hp(&self) -> u32 {
        55
    }

    pub fn mana(&self) -> u32 {
        10
    }

    pub fn stamina(&self) -> u32 {
        92
    }

    pub fn stat_points(&self) -> u16 {
        0
    }

    pub fn skill_points(&self) -> u8 {
        0
    }

    pub fn experience(&self) -> u32 {
        0
    }
}
