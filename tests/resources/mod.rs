use std::env;
use std::fs::File;
use std::io::{Error, Read};

pub fn read_test_resource(resource: &str) -> Result<Vec<u8>, Error> {
    let file_name = format!("{}.d2s", resource);

    let mut path = env::current_dir()?;
    path.push("tests");
    path.push("resources");
    path.push(file_name);

    let mut file = File::open(&path)?;
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
