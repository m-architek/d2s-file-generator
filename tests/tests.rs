mod resources;

use d2s_file_generator::character::Character;
use d2s_file_generator::character::class::Class;
use d2s_file_generator::character::difficulty::Difficulty;
use d2s_file_generator::character::mode::Mode;
use d2s_file_generator::generate_d2s;
use crate::resources::read_test_resource;

#[test]
fn should_generate_lvl1_hc_character() {
    // given
    let character = Character {
        name: String::from("Test"),
        class: Class::Barbarian,
        level: 1,
        mode: Mode::HC,
        completed_difficulty: None,
        last_played: u32::from_le_bytes([196, 191, 144, 99]),
        map_id: u32::from_le_bytes([95, 217, 87, 6])
    };

    // when
    let result = generate_d2s(&character);

    // then
    let expected = read_test_resource("Barbarian1_HC").unwrap();
    assert_eq!(result, expected)
}

#[test]
fn should_generate_lvl99_sc_character() {
    // given
    let character = Character {
        name: String::from("Test"),
        class: Class::Amazon,
        level: 99,
        mode: Mode::SC,
        completed_difficulty: None,
        last_played: u32::from_le_bytes([191, 121, 148, 99]),
        map_id: u32::from_le_bytes([170, 88, 77, 52])
    };

    // when
    let result = generate_d2s(&character);

    // then
    let expected = read_test_resource("Amazon99_SC").unwrap();
    assert_eq!(result, expected)
}

#[test]
fn should_generate_lvl35_hc_character_on_nm() {
    // given
    let character = Character {
        name: String::from("Test"),
        class: Class::Druid,
        level: 35,
        mode: Mode::HC,
        completed_difficulty: Some(Difficulty::NORMAL),
        last_played: u32::from_le_bytes([118, 5, 150, 99]),
        map_id: u32::from_le_bytes([180, 109, 39, 121])
    };

    // when
    let result = generate_d2s(&character);

    // then
    let expected = read_test_resource("Druid35_HC_NM").unwrap();
    assert_eq!(result, expected)
}
