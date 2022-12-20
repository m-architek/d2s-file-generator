use std::fmt::{Display, Formatter};
use std::ops::Deref;

use anyhow::{Error, Result};

#[derive(Debug)]
pub struct Level(u8);

impl TryFrom<u8> for Level {
    type Error = Error;

    fn try_from(number: u8) -> Result<Level> {
        if number < 1 {
            return Err(Error::msg("Character level has to be at least 2"))
        }
        if number > 99 {
            return Err(Error::msg("Character level has to be at most 99"))
        }
        Ok(Level(number))
    }
}

impl Deref<> for Level {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}
