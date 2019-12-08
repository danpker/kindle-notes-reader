use regex::Regex;

struct Note {
    book: String,
    author: String,
    text: String,
    raw_note: String,
}

impl Note {
    /// Create a note from the raw string input.
    ///
    /// This is the part of the 'My Clippings.txt' file after each note has
    /// been separated into individual notes.
    fn from_raw_string(raw_note: &str) -> Note {
        let lines: Vec<&str> = raw_note.trim().split("\n").collect();
        let re = Regex::new(r"(?P<title>.+)\((?P<author>.+)\)").unwrap();

        let book_info = match re.captures(lines[0]) {
            Some(re_match) => re_match,
            None => panic!(format!("Cannot find title and author in {}", lines[0])),
        };

        Note {
            book: book_info["title"].trim().to_string(),
            author: book_info["author"].trim().to_string(),
            text: lines.get(3).unwrap().to_string(),
            raw_note: raw_note.to_string(),
        }
    }
}

/// Split the 'My Clippings.txt' text by the note separator and create a note
/// for each note.
fn create_notes_from_string(clippings_text: &str) -> Vec<Note> {
    let mut output = Vec::new();
    for line in clippings_text.split("========") {
        dbg!(line);
        output.push(Note::from_raw_string(line));
    }
    output
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

    #[test]
    fn test_creates_vec_of_notes_from_clipping_file() {
        let clippings_text = "Title (Author)
Meta Data | Meta Data

I am the actual note
========
Title (Author)
Meta Data | Meta Data

I am the actual note
========
Title (Author)
Meta Data | Meta Data

I am the actual note";
        let notes = create_notes_from_string(clippings_text);

        assert_eq!(notes.len(), 3);
    }
}
