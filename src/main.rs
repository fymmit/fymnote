use fymnote::create_note;
use fymnote::timestamp::Timestamp;
use fymnote::{config::Config, file_parser};
use std::{
    env,
    error::Error,
    fs::{self},
    process::{self},
};

enum RunMode {
    Notes,
    Todos,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Problem reading config: {err}");
        process::exit(1);
    });

    let mut run_mode: Option<RunMode> = None;
    if args.len() > 1 {
        let arg = args[1].as_str();
        run_mode = match arg {
            "notes" => Some(RunMode::Notes),
            "todos" => Some(RunMode::Todos),
            _ => None,
        }
    }

    if let Err(e) = run(config, run_mode) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config, run_mode: Option<RunMode>) -> Result<(), Box<dyn Error>> {
    if let Ok(false) = fs::exists(&config.folder_path) {
        fs::create_dir(&config.folder_path).unwrap();
    }
    match run_mode {
        Some(mode) => {
            let notes = match mode {
                RunMode::Notes => file_parser::find_notes(config.folder_path)?,
                RunMode::Todos => file_parser::find_todos(config.folder_path)?,
            };
            for note in notes {
                println!("{note}");
            }
        }
        None => {
            let timestamp = Timestamp::new();
            create_note(
                config.editor,
                config.folder_path,
                timestamp.date,
                timestamp.time,
            )?;
        }
    }

    Ok(())
}
