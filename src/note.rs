use core::fmt;
use std::fmt::Display;

use crate::timestamp::Timestamp;

#[derive(Debug)]
pub struct Note {
    pub timestamp: Timestamp,
    pub content: String,
}

impl Note {
    fn mark_todo_as_done() {
        unimplemented!()
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
