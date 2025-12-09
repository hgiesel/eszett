#[derive(Debug)]
pub enum Language {
    English,
    French,
    Spanish,
    Italian,
    German,
    Mandarin,
    Japanese,
    Arabic,
}

// #[derive(Debug)]
// pub struct InvalidLanguage(u8);
//
// impl std::fmt::Display for InvalidLanguage {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Invalid language code: {}", self.0)
//     }
// }
//
// impl TryFrom<u8> for Language {
//     type Error = InvalidLanguage;
//     fn try_from(value: u8) -> Result<Self, Self::Error> {
//         match value {
//             0 => Ok(Language::English),
//             1 => Ok(Language::French),
//             2 => Ok(Language::Mandarin),
//             3 => Ok(Language::Arabic),
//             _ => Err(InvalidLanguage(value)),
//         }
//     }
// }
