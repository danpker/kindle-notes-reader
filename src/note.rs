struct Note {
    book: String,
    author: String,
    text: String,
    raw_note: String,
}

impl Note {
    fn from_raw_string(raw_note: &str) -> Note {
        let mut lines = raw_note.split("\n").map(|&x| x.to_string()).collect::<Vec<String>>();
        Note {
            book: get_book_title(lines.next().unwrap()),
            author:
            text: lines.nth(2).unwrap().to_string(),
            raw_note: raw_note.to_string(),
        }
    }
}

fn get_book_title(title_line: &str) -> String {
    // For now this is a really dump implementation.
    title_line.split("(").next().unwrap().trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sets_raw_note_to_raw_input() {
        let raw_note = "Title (Author)\nMeta Data | Meta Data\n\nI am the actual note.";
        let note = Note::from_raw_string(raw_note);

        assert_eq!(note.raw_note, raw_note);
    }

    #[test]
    fn test_gets_title_from_first_line() {
        let raw_note = "Title (Author)\nMeta Data | Meta Data\n\nI am the actual note.";
        let note = Note::from_raw_string(raw_note);

        assert_eq!(note.book, "Title");
    }

    #[test]
    fn test_gets_author_from_first_line() {
        let raw_note = "Title (Author)\nMeta Data | Meta Data\n\nI am the actual note.";
        let note = Note::from_raw_string(raw_note);

        assert_eq!(note.author, "Author");
    }

    #[test]
    fn test_gets_note_text_from_third_line() {
        let raw_note = "Title (Author)\nMeta Data | Meta Data\n\nI am the actual note.";
        let note = Note::from_raw_string(raw_note);

        assert_eq!(note.text, "I am the actual note.");
    }
}
