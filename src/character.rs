pub use class::Class;
pub use difficulty::Difficulty;
pub use mode::Mode;

mod class;
mod mode;
mod difficulty;

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
