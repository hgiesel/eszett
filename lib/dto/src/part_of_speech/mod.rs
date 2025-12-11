use std::fmt::Display;
use std::str::FromStr;
use serde::Deserialize;
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

// fn serialize_as_display<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
// where T: Display, S: Serializer {
//     serializer.serialize_str(&value.to_string())
// }
//
#[derive(PartialEq, Eq, Debug, Clone, Tsify)]
// #[tsify(namespace)]
pub enum PartOfSpeechDto {
    #[tsify(type = "0 | 1 | 2")]
    Noun,
    Verb,
    Adjective,
    Adverb,
}

#[test]
fn foo() {
    assert_eq!(PartOfSpeechDto::Noun.to_string(), "noun");
    assert_eq!(PartOfSpeechDto::Adjective.to_string(), "adj");
}

#[derive(Debug, Deserialize)]
pub struct InvalidPartOfSpeechCode(String);

impl Display for InvalidPartOfSpeechCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid part-of-speech: {}", self.0)
    }
}

impl FromStr for PartOfSpeechDto {
    type Err = InvalidPartOfSpeechCode;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noun" => Ok(PartOfSpeechDto::Noun),
            "verb" => Ok(PartOfSpeechDto::Verb),
            _ if s.len() >= 3 && "adjective".starts_with(s) => Ok(PartOfSpeechDto::Adjective),
            _ if s.len() >= 3 && "adverb".starts_with(s) => Ok(PartOfSpeechDto::Adverb),
            _ => Err(InvalidPartOfSpeechCode(s.to_string())),
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
    use super::PartOfSpeechDto;

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
