use std::{
    error::Error,
    io::{Write, stdout},
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute, queue,
    style::{self, Color},
    terminal,
};

use crate::note::Note;

pub fn browse_notes(notes: Vec<Note>) -> Result<(), Box<dyn Error>> {
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
                queue!(stdout(), style::SetForegroundColor(Color::Yellow))?;
            } else {
                queue!(stdout(), style::ResetColor)?;
            }
            queue!(stdout(), style::Print(&format!("{}\r\n", note)))?;
        }
        stdout().flush()?;
        match event::read()? {
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
