use crate::character::Character;
use crate::character::difficulty::Difficulty;

const DIFFICULTY_UNLOCKED: u8 = 0b1000_0000;

pub fn build_difficulty(character: &Character) -> [u8; 3]  {
    match &character.completed_difficulty {
        None => [DIFFICULTY_UNLOCKED, 0, 0],
        Some(difficulty) => match difficulty {
            Difficulty::NORMAL => [0, DIFFICULTY_UNLOCKED, 0],
            Difficulty::NIGHTMARE | Difficulty::HELL => [0, 0, DIFFICULTY_UNLOCKED]
        }
    }
}