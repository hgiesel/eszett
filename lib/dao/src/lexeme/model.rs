use crate::id::IdType;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]

pub struct LexemeId { pub id: IdType }

impl From<IdType> for LexemeId {
    fn from(id: IdType) -> Self {
        LexemeId { id }
    }
}