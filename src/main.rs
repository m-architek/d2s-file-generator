mod class;
mod mode;
mod character;
mod writer;

use class::Class;
use mode::Mode;
use character::Character;
use writer::FileWriter;

fn main() {
    let character = Character {
        name: String::from("Marcin"),
        class: Class::Barbarian,
        level: 65,
        mode: Mode::SC
    };

    println!("Generating {:#?}", character);

    let writer = FileWriter;
    let file = writer.write(character);

    println!("Result: {:?}", file)
}
