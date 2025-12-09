use std::fmt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de;
use serde::de::Visitor;
use base::language::Language;

#[derive(Debug)]
struct LanguageDto(Language);

impl From<Language> for LanguageDto {
    fn from(value: Language) -> Self {
        LanguageDto(value)
    }
}

////////

#[derive(Debug)]
pub struct InvalidLanguage(u8);

impl fmt::Display for InvalidLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid language code: {}", self.0)
    }
}

impl TryFrom<u8> for LanguageDto {
    type Error = InvalidLanguage;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Language::English.into()),
            1 => Ok(Language::French.into()),
            2 => Ok(Language::Mandarin.into()),
            3 => Ok(Language::Arabic.into()),
            _ => Err(InvalidLanguage(value)),
        }
    }
}

////////

impl Serialize for LanguageDto {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let LanguageDto(value) = self;
        let code = match value {
            Language::English => "en",
            Language::French  => "fr",
            Language::Mandarin => "zh",
            Language::Arabic => "ar",
            Language::Spanish => "es",
            Language::Italian => "it",
            Language::German => "de",
            Language::Japanese => "ja",
        };
        serializer.serialize_str(code)
    }
}
// -----------------------
struct LanguageVisitor;

impl<'de> Visitor<'de> for LanguageVisitor {
    type Value = LanguageDto;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(r#"a string: "en", "fr", "zh", or "ar""#)
    }

    fn visit_str<E>(self, value: &str) -> Result<LanguageDto, E>
    where
        E: de::Error,
    {
        match value {
            "en" => Ok(Language::English.into()),
            "fr" => Ok(Language::French.into()),
            "zh" => Ok(Language::Mandarin.into()),
            "ar" => Ok(Language::Arabic.into()),
            other => Err(E::custom(format!("invalid language code: {}", other))),
        }
    }
}

impl<'de> Deserialize<'de> for LanguageDto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(LanguageVisitor)
    }
}