use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Mode {
    SC,
    HC
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
