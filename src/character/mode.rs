use std::fmt::{Display, Formatter};
use anyhow::{anyhow, Error, Result};

#[derive(Debug)]
pub enum Mode {
    SC,
    HC
}

impl TryFrom<&str> for Mode {
    type Error = Error;

    fn try_from(value: &str) -> Result<Mode> {
        match value.to_uppercase().as_str() {
            "SC" => Ok(Mode::SC),
            "HC" => Ok(Mode::HC),
            _ => Err(anyhow!("Mode need to be SC or HC."))
        }
    }
}

impl TryFrom<String> for Mode {
    type Error = Error;

    fn try_from(value: String) -> Result<Mode> {
        value.as_str().try_into()
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
