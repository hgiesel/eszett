mod error;

use std::fmt::Display;
use std::str::FromStr;
use tsify::Tsify;
use error::InvalidLanguageCode;

#[derive(Tsify)]
#[tsify(namespace)]
pub enum LanguageDto {
    English,
    French,
    Spanish,
    Italian,
    German,
    Mandarin,
    Japanese,
    Arabic,
}

impl FromStr for LanguageDto {
    type Err = InvalidLanguageCode;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "en" => Ok(LanguageDto::English),
            "fr" => Ok(LanguageDto::French),
            "es" => Ok(LanguageDto::Spanish),
            "it" => Ok(LanguageDto::Italian),
            "de" => Ok(LanguageDto::German),
            "zh" => Ok(LanguageDto::Mandarin),
            "ja" => Ok(LanguageDto::Japanese),
            "ar" => Ok(LanguageDto::Arabic),
            _ => Err(InvalidLanguageCode(s.to_string())),
        }
    }
}

impl Display for LanguageDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            LanguageDto::English => "en",
            LanguageDto::French => "fr",
            LanguageDto::Spanish => "es",
            LanguageDto::Italian => "it",
            LanguageDto::German => "de",
            LanguageDto::Mandarin => "zh",
            LanguageDto::Japanese => "zh",
            LanguageDto::Arabic => "zh",
        }.to_string();
        write!(f, "{}", str)
    }
}
