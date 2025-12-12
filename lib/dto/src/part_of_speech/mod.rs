mod error;

use std::fmt::Display;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, Tsify)]
#[wasm_bindgen]
#[tsify(namespace)]
pub enum PartOfSpeechDto {
    #[serde(rename = "noun")]
    Noun,
    #[serde(rename = "verb")]
    Verb,
    #[serde(rename = "adj")]
    Adjective,
    #[serde(rename = "adv")]
    Adverb,
}

impl FromStr for PartOfSpeechDto {
    type Err = error::InvalidPartOfSpeechCode;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noun" => Ok(PartOfSpeechDto::Noun),
            "verb" => Ok(PartOfSpeechDto::Verb),
            _ if s.len() >= 3 && "adjective".starts_with(s) => Ok(PartOfSpeechDto::Adjective),
            _ if s.len() >= 3 && "adverb".starts_with(s) => Ok(PartOfSpeechDto::Adverb),
            _ => Err(error::InvalidPartOfSpeechCode(s.to_string())),
        }
    }
}

impl Display for PartOfSpeechDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            PartOfSpeechDto::Noun => "noun",
            PartOfSpeechDto::Verb => "verb",
            PartOfSpeechDto::Adjective => "adj",
            PartOfSpeechDto::Adverb => "adv",
        }.to_string();
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};
    use super::PartOfSpeechDto;

    #[test]
    fn display_as_string() {
        assert_eq!(PartOfSpeechDto::Noun.to_string(), "noun");
        assert_eq!(PartOfSpeechDto::Adjective.to_string(), "adj");
    }

    #[test]
    fn serialize_as_string() {
        assert_eq!(to_value(&PartOfSpeechDto::Noun).unwrap(), json!("noun"));
        assert_eq!(to_value(&PartOfSpeechDto::Adjective).unwrap(), json!("adj"));
    }

    #[test]
    fn can_match_adj_any_prefix() {
        assert_eq!("adj".parse::<PartOfSpeechDto>().unwrap(), PartOfSpeechDto::Adjective);
        assert_eq!("adje".parse::<PartOfSpeechDto>().unwrap(), PartOfSpeechDto::Adjective);
        assert_eq!("adjec".parse::<PartOfSpeechDto>().unwrap(), PartOfSpeechDto::Adjective);
        assert_eq!("adject".parse::<PartOfSpeechDto>().unwrap(), PartOfSpeechDto::Adjective);
        assert_eq!("adjecti".parse::<PartOfSpeechDto>().unwrap(), PartOfSpeechDto::Adjective);
        assert_eq!("adjectiv".parse::<PartOfSpeechDto>().unwrap(), PartOfSpeechDto::Adjective);
        assert_eq!("adjective".parse::<PartOfSpeechDto>().unwrap(), PartOfSpeechDto::Adjective);
    }

    #[test]
    fn no_invalid_adj_prefix() {
        assert!("ad".parse::<PartOfSpeechDto>().is_err());
        assert!(" adj".parse::<PartOfSpeechDto>().is_err());
        assert!("adjectives".parse::<PartOfSpeechDto>().is_err());
    }
}
