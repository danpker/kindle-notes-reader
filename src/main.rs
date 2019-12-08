use std::io::{self, Read};

mod note;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let notes = note::create_notes_from_string(&input);

    for note in notes {
        dbg!(note);
    }
}
