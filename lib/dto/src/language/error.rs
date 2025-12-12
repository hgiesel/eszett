use std::fmt::Display;

#[derive(Debug)]
pub struct InvalidLanguageCode(pub String);
impl Display for InvalidLanguageCode  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid language: {}", self.0)
    }
}

impl std::error::Error for InvalidLanguageCode {}
