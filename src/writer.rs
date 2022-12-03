use std::borrow::Borrow;
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::{Error, Write};

use crate::character::Character;

pub struct FileWriter;

impl FileWriter {

    pub fn write(&self, character: Character) -> Result<OsString, Error> {
        let file_name = format!("{}.d2s", character.name);
        let path = FileWriter::build_path(&file_name)?;
        let file = File::create(&path)?;
        self.write_character(file, character)?;
        Ok(path)
    }

    fn build_path(file_name: &str) -> Result<OsString, Error> {
        let mut path = env::current_dir()?;
        path.push("output");
        path.push(file_name);
        Ok(path.into_os_string())
    }

    fn write_character(&self, mut file: File, character: Character) -> Result<(), Error> {
        let bytes = self.build_bytes(character)?;
        file.write_all(bytes.borrow())
    }

    fn build_bytes(&self, character: Character) -> Result<Vec<u8>, Error> {
        let signature: u32 = 0xaa55aa55;
        let version_id: u32 = 96;

        let mut bytes: Vec<u8> = Vec::new();
        bytes.write(&signature.to_le_bytes())?;
        bytes.write(&version_id.to_le_bytes())?;
        Ok(bytes)
    }

    // func CalculateChecksum(data *[]byte) {
    // var sum uint32
    // for i := range *data {
    // sum = ((sum << 1) % math.MaxUint32) | (sum >> (int32Size*byteLen - 1))
    //
    // sum += uint32((*data)[i])
    // }
    //
    // sumBytes := make([]byte, int32Size)
    // binary.LittleEndian.PutUint32(sumBytes, sum)
    //
    // const (
    // int32Size = 4
    // checksumPosition = 12
    // )
    // for i := 0; i < int32Size; i++ {
    // (*data)[checksumPosition+i] = sumBytes[i]
    // }
    // }
}