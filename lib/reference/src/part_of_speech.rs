use std::fmt::Display;
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

impl Display for PartOfSpeech {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            PartOfSpeech::Noun => String::from("noun"),
            PartOfSpeech::Verb => String::from("verb"),
            PartOfSpeech::Adjective => String::from("adj"),
        };
        write!(f, "{}", str)
    }
}