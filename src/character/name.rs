use std::fmt::{Display, Formatter};
use std::ops::Deref;
use anyhow::{Error, Result};

#[derive(Debug)]
pub struct Name(String);

impl TryFrom<String> for Name {
    type Error = Error;

    /*
        Remember the rules for Diablo II character names: 2-15 characters,
        containing only upper and lower case letters (A-Z),
        with the possible addition of one dash ( - ) or underscore ( _ )
        as long as it is not the first or last character of the name.
    */
    fn try_from(str: String) -> Result<Name> {
        if str.len() < 2 {
            return Err(Error::msg("Character name has to be at least 2 characters length."))
        }
        if str.len() > 15 {
            return Err(Error::msg("Character name has to be at most 15 characters length."))
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
