mod resources;

use d2s_file_generator::{generate_d2s, Character, Class, Mode};
use crate::resources::read_test_resource;

#[test]
fn should_generate_lvl1_hc_character() {
    // given
    let character = Character {
        name: String::from("Test"),
        class: Class::Barbarian,
        level: 1,
        mode: Mode::HC,
        completed_difficulty: None
    };

    // when
    let result = generate_d2s(&character);

    // then
    let expected = read_test_resource("Barbarian1_HC").unwrap();
    assert_eq!(expected, result)
}