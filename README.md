# kindle-notes-reader
Reading Kindle notes and marking as added in Anki.

## Purpose

The purpose of this application is to parse and store notes exported from a
Kindle device via the "My Clippings.txt" file.

## Format

Each note takes the following format:
```
Football Hackers (Biermann, Christoph)
- Your Highlight on page 225 | location 2563-2565 | Added on Friday, 29 November 2019 22:20:37

His profile shows him being great at receiving balls in front of the back four (bypassed opponents receiving) and very good at producing through-balls that split the back line (bypassed defenders).
========
```

Each note is always separated by `========`.
