use std::fmt::{Display, Formatter};
use std::ops::Deref;
use anyhow::{Error, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NAME_PATTERN: Regex = Regex::new(r"^[a-zA-Z]+[-_]?[a-zA-Z]+$").unwrap();
}

#[derive(Debug)]
pub struct Name(String);

impl TryFrom<String> for Name {
    type Error = Error;

    fn try_from(str: String) -> Result<Name> {
        if str.len() < 2 {
            return Err(Error::msg("Character name has to be at least 2 characters length."))
        }
        if str.len() > 15 {
            return Err(Error::msg("Character name has to be at most 15 characters length."))
        }
        if !NAME_PATTERN.is_match(&str) {
            return Err(Error::msg("Character name can contain only letters with maximum one hyphen (-) or underscore (_), and need to start and end with a letter."))
        }
        Ok(Name(str))
    }
}

impl TryFrom<&str> for Name {
    type Error = Error;

    fn try_from(str: &str) -> Result<Name> {
        Name::try_from(str.to_string())
    }
}

impl Deref<> for Name {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self)
    }
}
