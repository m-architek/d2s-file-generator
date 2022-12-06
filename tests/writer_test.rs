use d2s_file_generator::{generate_d2s, Character, Class, Mode};

#[test]
fn should_generate_for_lvl1_character() {
    // given
    let character = Character {
        name: String::from("Marcin"),
        class: Class::Barbarian,
        level: 1,
        mode: Mode::SC,
        completed_difficulty: None
    };

    // when
    let result = generate_d2s(&character);

    // then
    assert!(result.is_ok());
    assert!(result.unwrap().len() > 0)
}