use std::error::Error;
use std::fs;

use crate::note::Note;
use crate::timestamp::Timestamp;

pub fn find_todos(folder_path: String) -> Result<Vec<Note>, Box<dyn Error>> {
    let all_notes = find_notes(folder_path)?;
    let todos = all_notes
        .into_iter()
        .filter(|note| note.content.contains("TODO:"))
        .collect();
    Ok(todos)
}

pub fn find_notes(folder_path: String) -> Result<Vec<Note>, Box<dyn Error>> {
    let mut result: Vec<Note> = vec![];
    let mut date = String::from("N/A");
    let mut time = String::from("N/A");
    for entry in fs::read_dir(folder_path)? {
        let file = entry?;
        if let Some(file_path) = file.path().to_str() {
            let file_content = fs::read_to_string(file_path)?;
            for line in file_content.lines() {
                if line.starts_with("## ") {
                    date = line.split("## ").skip(1).next().unwrap().to_string();
                } else if line.starts_with("# ") {
                    time = line.split("# ").skip(1).next().unwrap().to_string();
                } else if !line.is_empty() {
                    result.push(Note {
                        timestamp: Timestamp {
                            date: date.clone(),
                            time: time.clone(),
                        },
                        content: line.to_string(),
                    });
                }
            }
        }
    }
    Ok(result)
}
