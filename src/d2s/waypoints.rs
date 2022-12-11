use crate::Character;
use crate::character::difficulty::Difficulty;
use crate::utils::ArrayOverwrite;

const WAYPOINTS_EMPTY:[u8; 24] = [
    0x02, 0x01,
    0b0000_0001, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
];
const WAYPOINTS_COMPLETED:[u8; 24] = [
    0x02, 0x01,
    u8::MAX, u8::MAX, u8::MAX, u8::MAX, 0b0111_1111,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
];

pub fn build_waypoints(character: &Character) -> [u8; 81] {
    let mut waypoints: [u8; 81] = [0; 81];

    let header: [u8; 8] = [87, 83, 1, 0, 0, 0, 80, 0];
    let body: [[u8; 24]; 3] = match &character.completed_difficulty {
        None => [WAYPOINTS_EMPTY, WAYPOINTS_EMPTY, WAYPOINTS_EMPTY],
        Some(difficulty) => match difficulty {
            Difficulty::NORMAL => [WAYPOINTS_COMPLETED, WAYPOINTS_EMPTY, WAYPOINTS_EMPTY],
            Difficulty::NIGHTMARE => [WAYPOINTS_COMPLETED, WAYPOINTS_COMPLETED, WAYPOINTS_EMPTY],
            Difficulty::HELL => [WAYPOINTS_COMPLETED, WAYPOINTS_COMPLETED, WAYPOINTS_COMPLETED]
        }
    };

    waypoints.overwrite_with(&header, 0);
    waypoints.overwrite_with(&body.concat(), header.len());
    waypoints[80] = 1;
    waypoints
}
