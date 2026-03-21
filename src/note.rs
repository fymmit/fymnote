use crate::timestamp::Timestamp;

#[derive(Debug)]
pub struct Note {
    pub timestamp: Timestamp,
    pub content: String,
}
