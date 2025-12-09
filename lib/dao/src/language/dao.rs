use std::fmt;
use crate::id::IdType;

#[derive(PartialEq, Eq, Debug)]
pub enum LanguageDao {
    English = 1,
    Spanish,
    French,
    Italian,
}

#[derive(Debug)]
pub struct InvalidLanguageId(IdType);

impl fmt::Display for InvalidLanguageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid language id: {}", self.0)
    }
}

impl TryFrom<IdType> for LanguageDao {
    type Error = InvalidLanguageId;
    fn try_from(value: IdType) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(LanguageDao::English.into()),
            2 => Ok(LanguageDao::Spanish.into()),
            3 => Ok(LanguageDao::French.into()),
            4 => Ok(LanguageDao::Italian.into()),
            _ => Err(InvalidLanguageId(value)),
        }
    }
}

#[test]
fn serialize_language_dao() {
    let en = LanguageDao::English;
    let es = LanguageDao::Spanish;
    assert_eq!(1, en as i32);
    assert_eq!(2, es as i32);
}

#[test]
fn deserialize_language_dao() {
    let en: IdType = 1;
    let es: IdType = 2;
    let en_dao: LanguageDao = en.try_into().unwrap();
    let es_dao: LanguageDao = es.try_into().unwrap();
    assert_eq!(en_dao, LanguageDao::English);
    assert_eq!(es_dao, LanguageDao::Spanish);
}