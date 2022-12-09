use class::Class;
use difficulty::Difficulty;
use mode::Mode;

pub mod class;
pub mod mode;
pub mod difficulty;

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

    pub fn strength(&self) -> usize {
        30
    }
}
