use crate::character::Character;
use crate::character::difficulty::Difficulty;

const DIFFICULTY_UNLOCKED: u8 = 0b1000_0000;

pub fn build_difficulty(character: &Character) -> [u8; 3]  {
    match &character.completed_difficulty {
        None => [DIFFICULTY_UNLOCKED, 0, 0],
        Some(difficulty) => match difficulty {
            Difficulty::NORMAL => [DIFFICULTY_UNLOCKED, DIFFICULTY_UNLOCKED, 0],
            Difficulty::NIGHTMARE | Difficulty::HELL => [DIFFICULTY_UNLOCKED, DIFFICULTY_UNLOCKED, DIFFICULTY_UNLOCKED]
        }
    }
}