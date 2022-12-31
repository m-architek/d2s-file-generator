use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::{stdin, stdout, Write};
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
    println!("Diablo II save file generator");
    println!();

    let last_played = now();
    let map_id = rand::thread_rng().next_u32();

    let name = handle_input("Character name [2-15 letters]", Name::try_from)?;
    let class = handle_input(
        "Character class [1 (Amazon) / 2 (Sorceress) / 3 (Necromancer) / 4 (Paladin) / 5 (Barbarian) / 6 (Druid) / 7 (Assassin)]",
        Class::try_from
    )?;
    let mode = handle_input("Character mode [SC / HC]", Mode::try_from)?;
    let level = handle_input("Character level [1-99]", Level::try_from)?;
    let completed_difficulty = handle_input(
        "Completed difficulty [NONE / NORMAL / NIGHTMARE / HELL]",
        |input| Difficulty::resolve_completed_difficulty(&input, &level)
    )?;
    let gold = handle_input("Stashed gold [0-2500000]", Gold::try_from)?;

    let character = Character {
        name,
        class,
        level,
        mode,
        completed_difficulty,
        gold,
        last_played,
        map_id
    };

    let d2s = generate_d2s(&character);

    let path = build_path(&character.name)?;
    write_to_file(&path, &d2s)
}

fn handle_input<T, F>(message: &str, factory: F) -> Result<T>
    where F: Fn(String) -> Result<T>
{
    loop {
        let input_string = request_input(message)?;
        println!();

        let result = factory(input_string.clone());
        if result.is_ok() {
            break result
        }
        let error = result.err().unwrap();
        println!("Value '{input_string}' is invalid. {error}");
        println!();
    }
}

fn request_input(message: &str) -> Result<String> {
    print!("{message}: ");
    stdout().flush()?;

    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn build_path(character_name: &str) -> Result<OsString> {
    let file_name = format!("{character_name}.d2s");
    let mut path = env::current_dir()?;
    path.push(file_name);
    Ok(path.into_os_string())
}

fn write_to_file(path: &OsString, bytes: &Vec<u8>) -> Result<()> {
    println!("Creating file: {path:?}");
    File::options()
        .create_new(true)
        .write(true)
        .open(path)?
        .write_all(bytes)?;
    Ok(())
}

fn now() -> u32 {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH).unwrap().as_secs() as u32
}
