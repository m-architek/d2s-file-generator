use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::{Error, Write};
use d2s_file_generator::{Character, Class, generate_d2s, Mode};

fn main() -> Result<(), Error> {
    let character = Character {
        name: String::from("Marcin"),
        class: Class::Barbarian,
        level: 1,
        mode: Mode::SC,
        completed_difficulty: None
    };

    let d2s = generate_d2s(&character)?;

    let path = build_path(&character.name)?;
    write_to_file(&path, &d2s)
}

fn build_path(character_name: &str) -> Result<OsString, Error> {
    let file_name = format!("{}.d2s", character_name);
    let mut path = env::current_dir()?;
    path.push("output");
    path.push(file_name);
    Ok(path.into_os_string())
}

fn write_to_file(path: &OsString, bytes: &Vec<u8>) -> Result<(), Error> {
    println!("Creating file: {:?}", path);
    File::create(&path)?.write_all(bytes)
}
