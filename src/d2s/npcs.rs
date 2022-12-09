use crate::character::Character;
use crate::character::difficulty::Difficulty;
use crate::utils::ArrayOverwrite;

const INTRODUCTIONS_COMPLETED: [u8; 8] = [u8::MAX; 8];
const GREETINGS_COMPLETED: [u8; 8] = [u8::MAX; 8];

pub fn build_npcs(character: &Character) -> [u8; 51] {
    let mut npc_introductions: [u8; 51] = [0; 51];

    let header: [u8; 3] = [119, 52, 0];
    let body: [[u8; 8]; 6] = match &character.completed_difficulty {
        None => [[0; 8], [0; 8], [0; 8], [0; 8], [0; 8], [0; 8]],
        Some(difficulty) => match difficulty {
            Difficulty::NORMAL => [
                INTRODUCTIONS_COMPLETED, [0; 8], [0; 8],
                GREETINGS_COMPLETED, [0; 8], [0; 8]
            ],
            Difficulty::NIGHTMARE => [
                INTRODUCTIONS_COMPLETED, INTRODUCTIONS_COMPLETED, [0; 8],
                GREETINGS_COMPLETED, GREETINGS_COMPLETED, [0; 8]]
            ,
            Difficulty::HELL => [
                INTRODUCTIONS_COMPLETED, INTRODUCTIONS_COMPLETED, INTRODUCTIONS_COMPLETED,
                GREETINGS_COMPLETED, GREETINGS_COMPLETED, GREETINGS_COMPLETED
            ]
        }
    };

    npc_introductions.overwrite_with(&header, 0);
    npc_introductions.overwrite_with(&body.concat(), header.len());
    npc_introductions
}
