use crate::id::IdType;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]

pub struct PartOfSpeechId { pub id: IdType }

impl From<IdType> for PartOfSpeechId {
    fn from(id: IdType) -> Self {
        PartOfSpeechId { id }
    }
}