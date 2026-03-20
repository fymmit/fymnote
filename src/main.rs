use chrono::Local;
use std::{
    env,
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
    process::{self, Command},
};

fn main() {
    let editor = env::var("EDITOR").unwrap();

    let now = Local::now();
    let date = now.format("%Y-%m-%d").to_string();
    let timestamp = now.format("%H.%M").to_string();

    let folder_path = String::from("notes");

    if let Ok(false) = fs::exists(&folder_path) {
        fs::create_dir(&folder_path).unwrap();
    }

    if let Err(e) = create_note(editor, folder_path, date, timestamp) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn create_note(
    editor: String,
    folder_path: String,
    date: String,
    timestamp: String,
) -> Result<(), Box<dyn Error>> {
    let filename = format!("{folder_path}/{date}.txt");

    if let Ok(false) = fs::exists(&filename) {
        fs::File::create_new(&filename)?;
        let mut file = OpenOptions::new().append(true).open(&filename)?;
        file.write_all(format!("{date}\n").as_bytes())?;
    }

    let mut file = OpenOptions::new().append(true).open(&filename)?;

    file.write_all(format!("\n{timestamp}\n").as_bytes())?;

    Command::new(editor).arg(&filename).status()?;

    Ok(())
}
