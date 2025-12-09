use std::fmt;
use crate::id::IdType;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct LanguageId { pub id: IdType }

impl From<IdType> for LanguageId {
    fn from(id: IdType) -> Self {
        LanguageId { id }
    }
}