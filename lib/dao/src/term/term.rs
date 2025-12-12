use std::fmt::Display;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Term { pub term: String }

impl From<&str> for Term {
    fn from(value: &str) -> Self {
        Term { term: value.into() }
    }
}

impl Display for Term {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_str(self.term.as_str())
    }
}
