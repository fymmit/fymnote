use chrono::Local;

#[derive(Debug)]
pub struct Timestamp {
    pub date: String,
    pub time: String,
}

impl Timestamp {
    pub fn new() -> Timestamp {
        let now = Local::now();
        let date = now.format("%Y-%m-%d").to_string();
        let time = now.format("%H.%M").to_string();
        Timestamp { date, time }
    }
}
