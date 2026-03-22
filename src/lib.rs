pub mod config;
pub mod file_parser;
mod note;
pub mod timestamp;

use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
    process::Command,
};

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
