use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::read;
use crossterm::queue;
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::{execute, terminal};
use fymnote::config::Config;
use fymnote::file_parser;
use fymnote::note::Note;
use fymnote::timestamp::Timestamp;
use fymnote::{add_note, create_note, edit_notes};
use std::io::Write;
use std::io::{self, Read, stdout};
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
    terminal::enable_raw_mode()?;
    let mut selection = notes.len() - 1;

    let (_cols, rows) = terminal::size()?;
    let rows = usize::from(rows);

    execute!(stdout(), terminal::EnterAlternateScreen)?;
    loop {
        queue!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        for (i, note) in notes.iter().enumerate() {
            if i < selection.saturating_sub(rows) || i > selection {
                continue;
            }
            if i == selection {
                queue!(stdout(), SetForegroundColor(Color::Yellow))?;
            } else {
                queue!(stdout(), ResetColor)?;
            }
            queue!(stdout(), Print(&format!("{}\r\n", note)))?;
        }
        stdout().flush()?;
        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Char('k') => {
                    if selection > 0 {
                        selection -= 1
                    }
                }
                KeyCode::Char('j') => {
                    if selection < notes.len() - 1 {
                        selection += 1
                    }
                }
                KeyCode::Char('q') => break,
                _ => continue,
            },
            _ => break,
        }
    }
    execute!(stdout(), terminal::LeaveAlternateScreen)?;

    terminal::disable_raw_mode()?;
    Ok(())
}
