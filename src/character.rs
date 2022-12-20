use std::fmt::{Display, Formatter};

use class::Class;
use difficulty::Difficulty;
use mode::Mode;

use crate::character::experience::calculate_experience;
use crate::character::level::Level;
use crate::character::name::Name;

pub mod name;
pub mod class;
pub mod mode;
pub mod difficulty;
pub mod level;
mod experience;

#[derive(Debug)]
pub struct Character {
    pub name: Name,
    pub class: Class,
    pub level: Level,
    pub mode: Mode,
    pub completed_difficulty: Option<Difficulty>,
    pub gold: u32,
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
        self.class.base_hp() + self.level_ups() as f32 * self.class.level_up_hp()
            + self.game_completions() as f32 * 20.0
    }

    pub fn mana(&self) -> f32 {
        self.class.base_mana() + self.level_ups() as f32 * self.class.level_up_mana()
    }

    pub fn stamina(&self) -> f32 {
        self.class.base_stamina() + self.level_ups() as f32 * self.class.level_up_stamina()
    }

    pub fn stat_points(&self) -> u16 {
        self.level_ups() as u16 * 5 + self.game_completions() as u16 * 5
    }

    pub fn skill_points(&self) -> u8 {
        self.level_ups() + self.game_completions() * 4
    }

    pub fn experience(&self) -> u32 {
        calculate_experience(&self.level)
    }

    pub fn gold(&self) -> u32 {
        self.gold
    }

    fn level_ups(&self) -> u8 {
        *self.level - 1
    }

    fn game_completions(&self) -> u8 {
        match &self.completed_difficulty {
            None => 0,
            Some(difficulty) => match difficulty {
                Difficulty::NORMAL => 1,
                Difficulty::NIGHTMARE => 2,
                Difficulty::HELL => 3
            }
        }
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Character {
            name, class, mode, level, gold,
            completed_difficulty, ..
        } = self;
        write!(f, "{mode} {class} {name} level {level} with {gold} gold")?;

        match completed_difficulty {
            None => write!(f, " at the start of the game"),
            Some(difficulty) => write!(f, " after {:?}", difficulty)
        }
    }
}
