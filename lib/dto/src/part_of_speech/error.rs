use std::fmt::Display;

#[derive(Debug)]
pub struct InvalidPartOfSpeechCode(pub String);

impl Display for InvalidPartOfSpeechCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid part-of-speech: {}", self.0)
    }
}

impl std::error::Error for InvalidPartOfSpeechCode {}
