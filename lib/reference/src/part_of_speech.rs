use std::str::FromStr;
use anyhow::{anyhow, Error};

#[derive(Debug, Clone)]
pub enum PartOfSpeech {
    Noun,
    Verb,
    Adjective,
}

impl FromStr for PartOfSpeech {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noun" => Ok(PartOfSpeech::Noun),
            "verb" => Ok(PartOfSpeech::Verb),
            "adj" => Ok(PartOfSpeech::Adjective),
            _ => Err(anyhow!("Unknown English part-of-speech: {}", s)),
        }
    }
}
