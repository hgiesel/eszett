use crate::id::IdType;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct LemmaId { pub id: IdType }

impl From<IdType> for LemmaId {
    fn from(id: IdType) -> Self {
        LemmaId { id }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Lemma { pub term: String }

impl From<&str> for Lemma {
    fn from(term: &str) -> Self {
        Lemma { term: term.to_string() }
    }
}
