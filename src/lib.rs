use std::borrow::Borrow;

pub use character::*;

mod character;

pub fn generate_d2s(character: &Character) -> Vec<u8> {
    println!("Generating {:#?}", character);

    let signature: u32 = 0xaa55aa55;
    let version_id: u32 = 96;
    let file_size_temp: u32 = 0;
    let checksum_temp: u32 = 0;
    let active_weapon: u32 = 0;
    let character_name: [u8; 16] = {
        let mut bytes: [u8; 16] = [0; 16];
        for (i, char) in character.name.as_bytes().iter().enumerate() {
            bytes[i] = char.to_le();
        }
        bytes
    };
    let status: u8 = match character.mode {
        Mode::SC => 0,
        Mode::HC => 2
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
        Class::Amazon => 0x00,
        Class::Sorceress => 0x01,
        Class::Necromancer => 0x02,
        Class::Paladin => 0x03,
        Class::Barbarian => 0x04,
        Class::Druid => 0x05,
        Class::Assassin => 0x06
    };
    let level: u8 = character.level.unsigned_abs();
    let last_played: u32 = 0;
    let hotkeys: [u8; 64] = [0; 64];
    let mouse_buttons: [u8; 16] = [0; 16];
    let menu_appearance: [u8; 32] = [0; 32];
    let difficulty: [u8; 3] = match character.completed_difficulty.borrow() {
        None => [0_u8; 3],
        Some(difficulty) => match difficulty {
            Difficulty::NORMAL => [7, 0, 0],
            Difficulty::NIGHTMARE => [7, 7, 0],
            Difficulty::HELL => [7, 7, 7]
        }
    }.map(|it| it.to_le());
    let map_id: u32 = 0;
    let mercenary: [u8; 14] = [0; 14];
    let quests: [u8; 298] = [0; 298];
    let waypoints: [u8; 81] = [0; 81];
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
    bytes.extend(&[0; 2]);
    bytes.extend(&level.to_le_bytes());
    bytes.extend(&[0; 4]);
    bytes.extend(&last_played.to_le_bytes());
    bytes.extend(&[0; 4]);
    bytes.extend(&hotkeys);
    bytes.extend(&mouse_buttons);
    bytes.extend(&menu_appearance);
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