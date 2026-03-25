pub mod config;
pub mod file_parser;
pub mod note;
pub mod timestamp;

use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::{self, Write},
    process::Command,
};

use crate::{config::Config, timestamp::Timestamp};

// FIX: Improve code duplication around file path handling

pub fn add_note(config: Config, content: String) -> Result<(), Box<dyn Error>> {
    let mut content = content.clone();
    if content.trim().is_empty() {
        io::stdin().read_line(&mut content)?;
    }

    let timestamp = Timestamp::now();
    let folder_path = config.folder_path;
    let date = timestamp.date;

    let filename = format!("{folder_path}/{date}.txt");
    if let Ok(false) = fs::exists(&filename) {
        fs::File::create_new(&filename)?;
        let mut file = OpenOptions::new().append(true).open(&filename)?;
        file.write_all(format!("## {date}\n{content}").as_bytes())?;
    }

    let mut file = OpenOptions::new().append(true).open(&filename)?;
    let time = timestamp.time;

    // TODO: handle case where current time is the same as the previous entry. Time shouldn't be
    // inserted again in that case
    // ALTERNATIVE: actually use the timestamp as separator for notes instead of every note just
    // being a single line
    file.write_all(format!("\n# {time}\n{content}\n").as_bytes())?;

    Ok(())
}

pub fn edit_notes(config: Config) -> Result<(), Box<dyn Error>> {
    let editor = config.editor;
    let timestamp = Timestamp::now();
    let folder_path = config.folder_path;
    let date = timestamp.date;

    let file_path = format!("{folder_path}/{date}.txt");

    Command::new(editor).arg(file_path).status()?;

    Ok(())
}

pub fn create_note(
    editor: String,
    folder_path: String,
    date: String,
    timestamp: String,
) -> Result<(), Box<dyn Error>> {
    let filename = format!("{folder_path}/{date}.txt");

    if let Ok(false) = fs::exists(&filename) {
        fs::File::create_new(&filename)?;
        let mut file = OpenOptions::new().append(true).open(&filename)?;
        file.write_all(format!("## {date}\n").as_bytes())?;
    }

    let mut file = OpenOptions::new().append(true).open(&filename)?;

    file.write_all(format!("\n# {timestamp}\n").as_bytes())?;

    Command::new(editor).arg(&filename).status()?;

    Ok(())
}
