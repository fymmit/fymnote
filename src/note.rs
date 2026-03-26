use core::fmt;
use std::fmt::Display;

use crate::timestamp::Timestamp;

#[derive(Debug, PartialEq)]
pub struct Note {
    timestamp: Timestamp,
    pub content: String,
    file_name: String,
    line_number: usize,
}

impl Note {
    pub fn new(
        timestamp: Timestamp,
        content: String,
        file_name: String,
        line_number: usize,
    ) -> Self {
        Self {
            timestamp,
            content,
            file_name,
            line_number,
        }
    }
    pub fn file_location(&self) -> (&str, usize) {
        (&self.file_name, self.line_number)
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}: {}",
            self.timestamp.date, self.timestamp.time, self.content
        )
    }
}
