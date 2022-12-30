use std::fmt::{Display, Formatter};
use std::ops::Deref;

use anyhow::{Error, Result};

#[derive(Debug)]
pub struct Gold(u32);

impl Gold {
    pub const ZERO: Gold = Gold(0);
    pub const MAX: Gold = Gold(2_500_000);
}

impl TryFrom<u32> for Gold {
    type Error = Error;

    fn try_from(number: u32) -> Result<Gold> {
        if number > *Gold::MAX {
            return Err(Error::msg("Gold amount cannot be bigger then 2.5 million."))
        }
        Ok(Gold(number))
    }
}

impl TryFrom<String> for Gold {
    type Error = Error;

    fn try_from(value: String) -> Result<Gold> {
        let level_number: u32 = value.parse()?;
        Gold::try_from(level_number)
    }
}

impl Deref<> for Gold {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Gold {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}
