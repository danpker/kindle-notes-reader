use regex::Regex;

struct Note {
    book: String,
    author: String,
    text: String,
    raw_note: String,
}

impl Note {
    fn from_raw_string(raw_note: &str) -> Note {
        let mut lines: Vec<&str> = raw_note.split("\n").collect();
        let re = Regex::new(r"(?P<title>.+)\((?P<author>.+)\)").unwrap();
        let caps = re.captures(lines[0]).unwrap();
        Note {
            book: caps["title"].trim().to_string(),
            author: caps["author"].trim().to_string(),
            text: lines.get(3).unwrap().to_string(),
            raw_note: raw_note.to_string(),
        }
    }
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
