pub mod checksum;
pub mod quests;
pub mod waypoints;
pub mod npcs;
pub mod stats;
pub mod difficulty;

pub const EMPTY_HOTKEYS: [u8; 64] = [
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

pub const EMPTY_MOUSE_BUTTONS: [u8; 16] = [0; 16];

pub const EMPTY_SKILLS: [u8; 32] = [
    105, 102,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
];

pub const EMPTY_ITEMS: [u8; 13] = [74, 77, 0, 0, 74, 77, 0, 0, 106, 102, 107, 102, 0];
