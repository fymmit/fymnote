use crate::note::Note;

fn find_todos() -> Vec<Note> {
    unimplemented!()
}

fn find_notes_in_file(file_content: String) -> Vec<Note> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::timestamp::Timestamp;

    use super::*;

    #[test]
    fn parses_file_for_notes() {
        let file_content = String::from(
            "\
2026-01-01

09.09

foo
bar

10.10

fizz
buzz",
        );
        let result = find_notes_in_file(file_content);
        let expected = vec![
            Note {
                timestamp: Timestamp {
                    date: String::from("2026-01-01"),
                    time: String::from("09.28"),
                },
                content: String::from(
                    "\
foo
bar",
                ),
            },
            Note {
                timestamp: Timestamp {
                    date: String::from("2026-01-01"),
                    time: String::from("10.10"),
                },
                content: String::from(
                    "\
fizz
buzz",
                ),
            },
        ];
        assert_eq!(expected.len(), result.len());
        for i in 0..expected.len() {
            let expected_item = &expected[i];
            let result_item = &result[i];
            assert_eq!(expected_item.timestamp.date, result_item.timestamp.date);
            assert_eq!(expected_item.timestamp.time, result_item.timestamp.time);
            assert_eq!(expected_item.content, result_item.content);
        }
    }
}
