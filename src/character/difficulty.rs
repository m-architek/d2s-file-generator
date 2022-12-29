use std::fmt::{Display, Formatter};
use anyhow::{anyhow, Error, Result};
use crate::character::level::Level;

#[derive(Debug)]
pub enum Difficulty {
    NORMAL,
    NIGHTMARE,
    HELL
}

impl Difficulty {

    pub fn resolve_completed_difficulty(string: &str, level: &Level) -> Result<Option<Difficulty>> {
        match string.to_uppercase().as_str() {
            "NONE" => Ok(None),
            _ => Difficulty::try_from(string)
                .and_then(|it| it.validate_level(level))
                .map(Some)
        }
    }

    fn validate_level(self, level: &Level) -> Result<Difficulty> {
        let min_level = self.min_level();
        if *level < min_level {
            return Err(anyhow!("To complete difficulty {self} you need to be at least at level {min_level}."))
        }
        return Ok(self)
    }

    fn min_level(&self) -> Level {
        Level::try_from(match self {
            Difficulty::NORMAL => 20,
            Difficulty::NIGHTMARE => 40,
            Difficulty::HELL => 60
        }).unwrap()
    }
}

impl TryFrom<&str> for Difficulty {
    type Error = Error;

    fn try_from(value: &str) -> Result<Difficulty> {
        match value.to_uppercase().as_str() {
            "NORMAL" => Ok(Difficulty::NORMAL),
            "NIGHTMARE" => Ok(Difficulty::NIGHTMARE),
            "HELL" => Ok(Difficulty::HELL),
            _ => Err(Error::msg("Difficulty need to be NORMAL, NIGHTMARE or HELL."))
        }
    }
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
