use std::borrow::Borrow;
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::{Error, Write};

use crate::character::Character;
use crate::character::class::Class;
use crate::character::difficulty::Difficulty;
use crate::character::mode::Mode;

pub struct FileWriter;

impl FileWriter {

    pub fn write(&self, character: &Character) -> Result<OsString, Error> {
        let file_name = format!("{}.d2s", character.name);
        let path = FileWriter::build_path(&file_name)?;
        self.write_character(&path, character)?;
        Ok(path)
    }

    fn build_path(file_name: &str) -> Result<OsString, Error> {
        let mut path = env::current_dir()?;
        path.push("output");
        path.push(file_name);
        Ok(path.into_os_string())
    }

    fn write_character(&self, path: &OsString, character: &Character) -> Result<(), Error> {
        let bytes = self.build_bytes(character)?;
        let mut file = File::create(&path)?;
        file.write_all(bytes.borrow())
    }


}