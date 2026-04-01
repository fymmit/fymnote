use fymnote::config::Config;
use fymnote::file_parser;
use fymnote::note::Note;
use fymnote::timestamp::Timestamp;
use fymnote::{add_note, create_note, edit_notes};
use std::io::{self, Read};
use std::{
    env,
    error::Error,
    fs::{self},
    process::{self},
};

enum RunMode {
    Add(Vec<String>),
    Edit,
    Notes,
    Todos,
    Search,
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
            "add" => Some(RunMode::Add(args.into_iter().skip(2).collect())),
            "edit" => Some(RunMode::Edit),
            "notes" => Some(RunMode::Notes),
            "todos" => Some(RunMode::Todos),
            "search" => Some(RunMode::Search),
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
        fs::create_dir(&config.folder_path)?;
    }
    match run_mode {
        Some(mode) => {
            let notes = match mode {
                RunMode::Add(rest) => {
                    let content = rest.join(" ");
                    add_note(config, content)?;
                    None
                }
                RunMode::Edit => {
                    edit_notes(config)?;
                    None
                }
                RunMode::Notes => Some(file_parser::find_notes(config.folder_path)?),
                RunMode::Todos => Some(file_parser::find_todos(config.folder_path)?),
                RunMode::Search => unimplemented!(), // grep style thing
            };
            if let Some(notes) = notes {
                browse_notes(notes);
                // for note in notes {
                //     println!("{note}");
                // }
            }
        }
        None => {
            let timestamp = Timestamp::now();
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

fn browse_notes(notes: Vec<Note>) -> Result<(), Box<dyn Error>> {
    // TODO: use crossterm
    let mut selection = notes.len();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim() {
            "j" => println!("Down"),
            "k" => println!("Up"),
            "<Esc>" => break,
            _ => continue,
        }
    }

    Ok(())
}
