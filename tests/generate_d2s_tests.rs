use d2s_file_generator::character::Character;
use d2s_file_generator::character::class::Class;
use d2s_file_generator::character::difficulty::Difficulty;
use d2s_file_generator::character::mode::Mode;
use d2s_file_generator::character::name::Name;
use d2s_file_generator::generate_d2s;

use crate::resources::read_test_resource;

mod resources;

#[test]
fn should_generate_lvl1_hc_barbarian_character() {
    // given
    let character = Character {
        name: Name::try_from("Test").unwrap(),
        class: Class::Barbarian,
        level: 1,
        mode: Mode::HC,
        completed_difficulty: None,
        gold: 0,
        last_played: u32::from_le_bytes([196, 191, 144, 99]),
        map_id: u32::from_le_bytes([95, 217, 87, 6])
    };

    // when
    let result = generate_d2s(&character);

    // then
    let expected = read_test_resource("Barbarian_1_HC").unwrap();
    assert_eq!(result, expected)
}

#[test]
fn should_generate_lvl1_sc_amazon_character_with_gold() {
    // given
    let character = Character {
        name: Name::try_from("Test").unwrap(),
        class: Class::Amazon,
        level: 1,
        mode: Mode::SC,
        completed_difficulty: None,
        gold: 1,
        last_played: u32::from_le_bytes([91, 11, 159, 99]),
        map_id: u32::from_le_bytes([150, 111, 144, 87])
    };

    // when
    let result = generate_d2s(&character);

    // then
    let expected = read_test_resource("Amazon_1_SC_GOLD").unwrap();
    assert_eq!(result, expected)
}

#[test]
fn should_generate_lvl99_sc_amazon_character() {
    // given
    let character = Character {
        name: Name::try_from("Test").unwrap(),
        class: Class::Amazon,
        level: 99,
        mode: Mode::SC,
        completed_difficulty: None,
        gold: 0,
        last_played: u32::from_le_bytes([191, 121, 148, 99]),
        map_id: u32::from_le_bytes([170, 88, 77, 52])
    };

    // when
    let result = generate_d2s(&character);

    // then
    let expected = read_test_resource("Amazon_99_SC").unwrap();
    assert_eq!(result, expected)
}

#[test]
fn should_generate_lvl2_hc_barbarian_character_with_gold() {
    // given
    let character = Character {
        name: Name::try_from("Test").unwrap(),
        class: Class::Barbarian,
        level: 2,
        mode: Mode::HC,
        completed_difficulty: None,
        gold: 1,
        last_played: u32::from_le_bytes([187, 142, 160, 99]),
        map_id: u32::from_le_bytes([84, 63, 179, 49])
    };

    // when
    let result = generate_d2s(&character);

    // then
    let expected = read_test_resource("Barbarian_2_HC_GOLD").unwrap();
    assert_eq!(result, expected)
}

#[test]
fn should_generate_lvl35_hc_druid_character_on_nightmare() {
    // given
    let character = Character {
        name: Name::try_from("Test").unwrap(),
        class: Class::Druid,
        level: 35,
        mode: Mode::HC,
        completed_difficulty: Some(Difficulty::NORMAL),
        gold: 0,
        last_played: u32::from_le_bytes([118, 5, 150, 99]),
        map_id: u32::from_le_bytes([180, 109, 39, 121])
    };

    // when
    let result = generate_d2s(&character);

    // then
    let expected = read_test_resource("Druid_35_HC_NM").unwrap();
    assert_eq!(result, expected)
}

#[test]
fn should_generate_lv65_sc_assassin_character_on_hell() {
    // given
    let character = Character {
        name: Name::try_from("Test").unwrap(),
        class: Class::Assassin,
        level: 65,
        mode: Mode::SC,
        completed_difficulty: Some(Difficulty::NIGHTMARE),
        gold: 0,
        last_played: u32::from_le_bytes([210, 91, 155, 99]),
        map_id: u32::from_le_bytes([220, 43, 226, 82])
    };

    // when
    let result = generate_d2s(&character);

    // then
    let expected = read_test_resource("Assassin_65_SC_HELL").unwrap();
    assert_eq!(result, expected)
}

#[test]
fn should_generate_lv85_hc_necromancer_character_after_hell() {
    // given
    let character = Character {
        name: Name::try_from("Test").unwrap(),
        class: Class::Necromancer,
        level: 85,
        mode: Mode::HC,
        completed_difficulty: Some(Difficulty::HELL),
        gold: 0,
        last_played: u32::from_le_bytes([226, 92, 155, 99]),
        map_id: u32::from_le_bytes([214, 21, 87, 69])
    };

    // when
    let result = generate_d2s(&character);

    // then
    let expected = read_test_resource("Necromancer_85_HC_COMPLETE").unwrap();
    assert_eq!(result, expected)
}

#[test]
fn should_generate_lvl35_sc_paladin_character_with_gold_on_nightmare() {
    // given
    let character = Character {
        name: Name::try_from("Test").unwrap(),
        class: Class::Paladin,
        level: 35,
        mode: Mode::SC,
        completed_difficulty: Some(Difficulty::NORMAL),
        gold: 2500000,
        last_played: u32::from_le_bytes([178, 93, 155, 99]),
        map_id: u32::from_le_bytes([143, 162, 162, 79])
    };

    // when
    let result = generate_d2s(&character);

    // then
    let expected = read_test_resource("Paladin_35_SC_NM_GOLD").unwrap();
    assert_eq!(result, expected)
}

#[test]
fn should_generate_lv65_hc_sorceress_character_with_gold_on_hell() {
    // given
    let character = Character {
        name: Name::try_from("Test").unwrap(),
        class: Class::Sorceress,
        level: 65,
        mode: Mode::HC,
        completed_difficulty: Some(Difficulty::NIGHTMARE),
        gold: 2500000,
        last_played: u32::from_le_bytes([173, 94, 155, 99]),
        map_id: u32::from_le_bytes([151, 198, 160, 66])
    };

    // when
    let result = generate_d2s(&character);

    // then
    let expected = read_test_resource("Sorceress_65_HC_HELL_GOLD").unwrap();
    assert_eq!(result, expected)
}

#[test]
fn should_generate_lv85_sc_barbarian_character_with_gold_after_hell() {
    // given
    let character = Character {
        name: Name::try_from("Test").unwrap(),
        class: Class::Barbarian,
        level: 85,
        mode: Mode::SC,
        completed_difficulty: Some(Difficulty::HELL),
        gold: 2500000,
        last_played: u32::from_le_bytes([94, 95, 155, 99]),
        map_id: u32::from_le_bytes([18, 96, 93, 87])
    };

    // when
    let result = generate_d2s(&character);

    // then
    let expected = read_test_resource("Barbarian_85_SC_COMPLETE_GOLD").unwrap();
    assert_eq!(result, expected)
}
