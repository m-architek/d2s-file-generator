use class::Class;
use difficulty::Difficulty;
use mode::Mode;
use crate::character::experience::calculate_experience;

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
        self.class.base_strength()
    }

    pub fn dexterity(&self) -> u16 {
        self.class.base_dexterity()
    }

    pub fn vitality(&self) -> u16 {
        self.class.base_vitality()
    }

    pub fn energy(&self) -> u16 {
        self.class.base_energy()
    }

    pub fn hp(&self) -> f32 {
        self.class.base_hp() + (self.level - 1) as f32 * self.class.level_hp()
    }

    pub fn mana(&self) -> f32 {
        self.class.base_mana() + (self.level - 1) as f32 * self.class.level_mana()
    }

    pub fn stamina(&self) -> f32 {
        self.class.base_stamina() + (self.level - 1) as f32 * self.class.level_stamina()
    }

    pub fn stat_points(&self) -> u8 {
        self.level - 1
    }

    pub fn skill_points(&self) -> u8 {
        self.level - 1
    }

    pub fn experience(&self) -> u32 {
        calculate_experience(self.level)
    }

    pub fn gold(&self) -> u32 {
        0
    }
}
