use std::{env, error::Error, fs::{self, OpenOptions}, io::Write, process::{self, Command}};
use chrono::{ Local };

fn main() {
    let editor = env::var("EDITOR").unwrap();

    let now = Local::now();
    let date = now.format("%Y-%m-%d");
    let timestamp = now.format("%H.%M").to_string();

    let note_folder = "notes";
    if let Ok(false) = fs::exists(note_folder) {
        fs::create_dir(note_folder).unwrap();
    }

    let filename = format!("{note_folder}/{}.txt", date.to_string());

    if let Err(e) = take_notes(editor, filename, timestamp) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn take_notes(editor: String, filename: String, timestamp: String) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&filename)?;

    file.write_all(format!("{timestamp}\n").as_bytes())?;

    _ = Command::new(editor).arg(&filename).status()?;

    Ok(())
}
