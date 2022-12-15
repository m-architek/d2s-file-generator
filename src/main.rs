use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::{Error, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use d2s_file_generator::character::Character;
use d2s_file_generator::character::class::Class;
use d2s_file_generator::character::difficulty::Difficulty;
use d2s_file_generator::character::mode::Mode;
use d2s_file_generator::generate_d2s;

fn main() -> Result<(), Error> {
    let character = Character {
        name: String::from("Marcin"),
        class: Class::Paladin,
        level: 65,
        mode: Mode::SC,
        completed_difficulty: Some(Difficulty::NIGHTMARE),
        gold: 2500000,
        last_played: now(),
        map_id: 0
    };

    let d2s = generate_d2s(&character);

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

pub fn now() -> u32 {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH).unwrap().as_secs() as u32
}
