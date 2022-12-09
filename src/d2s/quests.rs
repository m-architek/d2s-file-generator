use crate::Character;
use crate::character::difficulty::Difficulty;
use crate::utils::ArrayOverwrite;

const QUESTS_COMPLETED:[u8; 96] = [u8::MAX; 96];

pub fn build_quests(character: &Character) -> [u8; 298] {
    let mut quests: [u8; 298] = [0; 298];

    let header: [u8; 10] = [87, 111, 111, 33, 6, 0, 0, 0, 42, 1];
    let body: [[u8; 96]; 3] = match &character.completed_difficulty {
        None => [[0; 96], [0; 96], [0; 96]],
        Some(difficulty) => match difficulty {
            Difficulty::NORMAL => [QUESTS_COMPLETED, [0; 96], [0; 96]],
            Difficulty::NIGHTMARE => [QUESTS_COMPLETED, QUESTS_COMPLETED, [0; 96]],
            Difficulty::HELL => [QUESTS_COMPLETED, QUESTS_COMPLETED, QUESTS_COMPLETED]
        }
    };

    quests.overwrite_with(&header, 0);
    quests.overwrite_with(&body.concat(), header.len());
    quests
}
