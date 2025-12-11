use crate::id::IdType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TermId { pub id: IdType }

impl From<IdType> for TermId {
    fn from(id: IdType) -> Self {
        TermId { id }
    }
}
