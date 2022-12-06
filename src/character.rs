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
    pub level: i8,
    pub mode: Mode,
    pub completed_difficulty: Option<Difficulty>
}
