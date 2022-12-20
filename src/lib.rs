use crate::character::Character;
use crate::character::class::Class;
use crate::character::difficulty::Difficulty;
use crate::character::mode::Mode;
use crate::d2s::{EMPTY_HOTKEYS, EMPTY_MOUSE_BUTTONS, EMPTY_ITEMS, EMPTY_SKILLS};
use crate::d2s::checksum::calculate_checksum;
use crate::d2s::difficulty::build_difficulty;
use crate::d2s::npcs::build_npcs;
use crate::d2s::quests::build_quests;
use crate::d2s::stats::build_stats;
use crate::d2s::waypoints::build_waypoints;
use crate::utils::WithPadding;

pub mod character;

mod utils;
mod d2s;

pub fn generate_d2s(character: &Character) -> Vec<u8> {
    println!("Generating character: {character}.");

    let signature: u32 = 0xaa55aa55;
    let version_id: u32 = 96;
    let file_size_placeholder: u32 = 0;
    let checksum_placeholder: u32 = 0;
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
    let level: u8 = *character.level;
    let last_played: u32 = character.last_played;
    let difficulty: [u8; 3] = build_difficulty(character);
    let map_id: u32 = character.map_id;
    let mercenary: [u8; 14] = [0; 14];
    let quests: [u8; 298] = build_quests(character);
    let waypoints: [u8; 81] = build_waypoints(character);
    let npcs: [u8; 51] = build_npcs(character);
    let stats: Vec<u8> = build_stats(character);

    let mut bytes: Vec<u8> = Vec::new();
    bytes.extend(signature.to_le_bytes());
    bytes.extend(&version_id.to_le_bytes());
    bytes.extend(&file_size_placeholder.to_le_bytes());
    bytes.extend(&checksum_placeholder.to_le_bytes());
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
    bytes.extend(&EMPTY_HOTKEYS);
    bytes.extend(&EMPTY_MOUSE_BUTTONS);
    bytes.extend(&[u8::MAX; 32]);
    bytes.extend(&difficulty);
    bytes.extend(&map_id.to_le_bytes());
    bytes.extend(&[0; 2]);
    bytes.extend(&mercenary);
    bytes.extend(&[0; 144]);
    bytes.extend(&quests);
    bytes.extend(&waypoints);
    bytes.extend(&npcs);
    bytes.extend(&stats);
    bytes.extend(&EMPTY_SKILLS);
    bytes.extend(&EMPTY_ITEMS);

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
