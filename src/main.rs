use character::Character;
use class::Class;
use difficulty::Difficulty;
use mode::Mode;
use writer::FileWriter;

mod class;
mod mode;
mod character;
mod writer;
mod difficulty;

fn main() {
    let character = Character {
        name: String::from("Marcin"),
        class: Class::Barbarian,
        level: 1,
        mode: Mode::SC,
        completed_difficulty: None
    };

    println!("Generating {:#?}", character);

    let writer = FileWriter;
    let file = writer.write(&character);

    println!("Result: {:?}", file)
}
