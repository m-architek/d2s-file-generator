use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use rand;
use rand::RngCore;

use d2s_file_generator::character::Character;
use d2s_file_generator::character::class::Class;
use d2s_file_generator::character::difficulty::Difficulty;
use d2s_file_generator::character::gold::Gold;
use d2s_file_generator::character::level::Level;
use d2s_file_generator::character::mode::Mode;
use d2s_file_generator::character::name::Name;
use d2s_file_generator::generate_d2s;

fn main() -> Result<()> {
    let mut rng = rand::thread_rng();

    let character = Character {
        name: Name::try_from("Marcin")?,
        class: Class::Paladin,
        level: Level::try_from(65)?,
        mode: Mode::SC,
        completed_difficulty: Some(Difficulty::NIGHTMARE),
        gold: Gold::try_from(2500000)?,
        last_played: now(),
        map_id: rng.next_u32()
    };

    let d2s = generate_d2s(&character);

    let path = build_path(&character.name)?;
    write_to_file(&path, &d2s)
}

fn build_path(character_name: &str) -> Result<OsString> {
    let file_name = format!("{character_name}.d2s");
    let mut path = env::current_dir()?;
    path.push("output");
    path.push(file_name);
    Ok(path.into_os_string())
}

fn write_to_file(path: &OsString, bytes: &Vec<u8>) -> Result<()> {
    println!("Creating file: {path:?}");
    File::create(&path)?.write_all(bytes)?;
    Ok(())
}

fn now() -> u32 {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH).unwrap().as_secs() as u32
}
