use crate::class::Class;
use crate::difficulty::Difficulty;
use crate::mode::Mode;

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub class: Class,
    pub level: i8,
    pub mode: Mode,
    pub completed_difficulty: Option<Difficulty>
}