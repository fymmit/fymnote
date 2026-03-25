use core::fmt;
use std::fmt::Display;

use crate::timestamp::Timestamp;

#[derive(Debug)]
pub struct Note {
    timestamp: Timestamp,
    pub content: String,
    file_name: String,
    line_number: u16,
}

impl Note {
    pub fn new(timestamp: Timestamp, content: String, file_name: String, line_number: u16) -> Self {
        Self {
            timestamp,
            content,
            file_name,
            line_number,
        }
    }
    pub fn get_file_location(&self) -> (String, u16) {
        (self.file_name.clone(), self.line_number)
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
