pub use character::*;
use crate::utils::{ArrayOverwrite, WithPadding};

mod character;
mod utils;

pub fn generate_d2s(character: &Character) -> Vec<u8> {
    println!("Generating {:#?}", character);

    let signature: u32 = 0xaa55aa55;
    let version_id: u32 = 96;
    let file_size_temp: u32 = 0;
    let checksum_temp: u32 = 0;
    let active_weapon: u32 = 0;
    let character_name: [u8; 16] = character.name.with_padding();
    let status: u8 = match character.mode {
        Mode::SC => 0b0010_0000,
        Mode::HC => 0b0010_0100
    };
    let progression: u8 = match &character.completed_difficulty {
        None => 0,
        Some(difficulty) => match difficulty {
            Difficulty::NORMAL => 5,
            Difficulty::NIGHTMARE => 10,
            Difficulty::HELL => 15
        }
    };
    let class: u8 = match character.class {
        Class::Amazon => 0,
        Class::Sorceress => 1,
        Class::Necromancer => 2,
        Class::Paladin => 3,
        Class::Barbarian => 4,
        Class::Druid => 5,
        Class::Assassin => 6
    };
    let level: u8 = character.level;
    let last_played: u32 = character.last_played;
    let difficulty: [u8; 3] = match &character.completed_difficulty {
        None => [DIFFICULTY_UNLOCKED, 0, 0],
        Some(difficulty) => match difficulty {
            Difficulty::NORMAL => [DIFFICULTY_UNLOCKED, DIFFICULTY_UNLOCKED, 0],
            Difficulty::NIGHTMARE | Difficulty::HELL => [DIFFICULTY_UNLOCKED, DIFFICULTY_UNLOCKED, DIFFICULTY_UNLOCKED]
        }
    };
    let map_id: u32 = character.map_id;
    let mercenary: [u8; 14] = [0; 14];
    let quests: [u8; 298] = build_quests(character);
    let waypoints: [u8; 81] = build_waypoints(character);
    let npc_introductions: [u8; 51] = [0; 51];
    let stats: Vec<u8> = Vec::new();

    let mut bytes: Vec<u8> = Vec::new();
    bytes.extend(signature.to_le_bytes());
    bytes.extend(&version_id.to_le_bytes());
    bytes.extend(&file_size_temp.to_le_bytes());
    bytes.extend(&checksum_temp.to_le_bytes());
    bytes.extend(&active_weapon.to_le_bytes());
    bytes.extend(&character_name);
    bytes.extend(&status.to_le_bytes());
    bytes.extend(&progression.to_le_bytes());
    bytes.extend(&[0; 2]);
    bytes.extend(&class.to_le_bytes());
    bytes.extend(&[16, 30]);
    bytes.extend(&level.to_le_bytes());
    bytes.extend(&[0; 4]);
    bytes.extend(&last_played.to_le_bytes());
    bytes.extend(&[u8::MAX; 4]);
    bytes.extend(&DEFAULT_HOTKEYS);
    bytes.extend(&DEFAULT_MOUSE_BUTTONS);
    bytes.extend(&[u8::MAX; 32]);
    bytes.extend(&difficulty);
    bytes.extend(&map_id.to_le_bytes());
    bytes.extend(&[0; 2]);
    bytes.extend(&mercenary);
    bytes.extend(&[0; 144]);
    bytes.extend(&quests);
    bytes.extend(&waypoints);
    bytes.extend(&npc_introductions);
    bytes.extend(&stats);

    let file_size: [u8; 4] = (bytes.len() as u32).to_le_bytes();
    bytes[8] = file_size[0];
    bytes[9] = file_size[1];
    bytes[10] = file_size[2];
    bytes[11] = file_size[3];

    let checksum: [u8; 4] = calculate_checksum(&bytes).to_le_bytes();
    bytes[12] = checksum[0];
    bytes[13] = checksum[1];
    bytes[14] = checksum[2];
    bytes[15] = checksum[3];

    bytes
}

fn calculate_checksum(bytes: &Vec<u8>) -> u32 {
    let mut sum: u32 = 0;
    for byte in bytes {
        sum = sum.rotate_left(1);
        sum += Into::<u32>::into(*byte);
    }
    sum
}

fn build_quests(character: &Character) -> [u8; 298] {
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

fn build_waypoints(character: &Character) -> [u8; 81] {
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
    waypoints
}

const DEFAULT_HOTKEYS: [u8; 64] = [
    255, 255, 0, 0,
    255, 255, 0, 0,
    255, 255, 0, 0,
    255, 255, 0, 0,
    255, 255, 0, 0,
    255, 255, 0, 0,
    255, 255, 0, 0,
    255, 255, 0, 0,
    255, 255, 0, 0,
    255, 255, 0, 0,
    255, 255, 0, 0,
    255, 255, 0, 0,
    255, 255, 0, 0,
    255, 255, 0, 0,
    255, 255, 0, 0,
    255, 255, 0, 0,
];

const DEFAULT_MOUSE_BUTTONS: [u8; 16] = [0; 16];

const DIFFICULTY_UNLOCKED: u8 = 0b1000_0000;

const QUESTS_COMPLETED:[u8; 96] = [u8::MAX; 96];

const WAYPOINTS_EMPTY:[u8; 24] = [
    0x02, 0x01,
    0b0000_0001, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
];
const WAYPOINTS_COMPLETED:[u8; 24] = [
    0x02, 0x01,
    u8::MAX, u8::MAX, u8::MAX, u8::MAX, 0b0011_1111,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
];
