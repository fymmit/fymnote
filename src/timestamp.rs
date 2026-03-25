use chrono::Local;

#[derive(Debug)]
pub struct Timestamp {
    pub date: String,
    pub time: String,
}

impl Timestamp {
    pub fn new(date: String, time: String) -> Self {
        Self { date, time }
    }
    pub fn now() -> Self {
        let now = Local::now();
        let date = now.format("%Y-%m-%d").to_string();
        let time = now.format("%H.%M").to_string();
        Self { date, time }
    }
}
