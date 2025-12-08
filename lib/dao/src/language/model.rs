use crate::id::IdType;

pub struct LanguageId { pub id: IdType }

impl From<IdType> for LanguageId {
    fn from(id: IdType) -> Self {
        LanguageId { id }
    }
}