use fymnote::config::Config;
use fymnote::create_note;
use fymnote::timestamp::Timestamp;
use std::{
    fs::{self},
    process::{self},
};

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprint!("Problem reading config: {err}");
        process::exit(1);
    });

    let timestamp = Timestamp::new();

    if let Ok(false) = fs::exists(&config.folder_path) {
        fs::create_dir(&config.folder_path).unwrap();
    }

    if let Err(e) = create_note(
        config.editor,
        config.folder_path,
        timestamp.date,
        timestamp.time,
    ) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
