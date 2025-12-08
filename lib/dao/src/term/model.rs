use crate::id::IdType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TermId { pub id: IdType }

impl From<IdType> for TermId {
    fn from(id: IdType) -> Self {
        TermId { id }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Term { pub term: String }

impl From<&str> for Term {
    fn from(term: &str) -> Self {
        Term { term: term.to_string() }
    }
}