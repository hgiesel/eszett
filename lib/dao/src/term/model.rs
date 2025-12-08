#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TermId { pub id: i32 }

impl From<i32> for TermId {
    fn from(id: i32) -> Self {
        TermId { id }
    }
}
