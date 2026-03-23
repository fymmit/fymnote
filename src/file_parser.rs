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
    for entry in fs::read_dir(folder_path)? {
        let file = entry?;
        if let Some(file_path) = file.path().to_str() {
            let file_content = fs::read_to_string(file_path)?;
            let notes_in_file = find_notes_from_file_content(file_content);
            result.extend(notes_in_file);
        }
    }
    Ok(result)
}

fn find_notes_from_file_content(file_content: String) -> Vec<Note> {
    let mut result: Vec<Note> = vec![];
    let mut date = String::from("N/A");
    let mut time = String::from("N/A");
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
    result
}

#[cfg(test)]
mod tests {
    use crate::{file_parser::find_notes_from_file_content, note::Note, timestamp::Timestamp};

    #[test]
    fn finds_notes_from_file_content() {
        let file_content = String::from(
            "\
## 2026-03-22

# 08.14
test content

# 08.26
foo bar

# 08.43
fizz buzz",
        );

        let expected_notes: Vec<Note> = vec![
            Note {
                timestamp: Timestamp {
                    date: String::from("2026-03-22"),
                    time: String::from("08.14"),
                },
                content: String::from("test content"),
            },
            Note {
                timestamp: Timestamp {
                    date: String::from("2026-03-22"),
                    time: String::from("08.26"),
                },
                content: String::from("foo bar"),
            },
            Note {
                timestamp: Timestamp {
                    date: String::from("2026-03-22"),
                    time: String::from("08.43"),
                },
                content: String::from("fizz buzz"),
            },
        ];

        let result_notes = find_notes_from_file_content(file_content);

        for (i, item) in expected_notes.iter().enumerate() {
            let expected = &expected_notes[i];
            let result = &result_notes[i];
            assert_eq!(format!("{expected}"), format!("{result}"));
        }
    }
}
