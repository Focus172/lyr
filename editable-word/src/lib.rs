#![feature(round_char_boundary)]

//! A library for editing words in a terminal
//! This has been useful for me when dealing with 
//! non-utf8 characters in the tui programs as hadling 
//! them as individual bytes is not a good idea as it
//! can result in panics.
//!
//! This aims to be a safe wrapper arond these to allow
//! them to be safely edited with minimal effort.

pub struct EditableWord {
    pub word: String,
    pub cursor: usize,
}

// Note this code crashes on non-utf8 characters
// as it trys to remove a byte from inside a character
impl EditableWord {
    pub fn new(word: String) -> Self {
        let cursor = word.len();
        Self { word, cursor }
    }

    pub fn add(&mut self, c: char) {
        if self.cursor < self.word.len() {
            self.word.insert(self.cursor, c);
        } else {
            self.word.push(c);
        }
        self.cursor += 1;
    }

    /// Removes the character that the cursor is over regardless of
    /// its size in bytes
    pub fn del(&mut self) {
        let slice = self.word.as_str();
        let bot = slice.floor_char_boundary(self.cursor.saturating_sub(1));
        let top = slice.ceil_char_boundary(self.cursor);

        let start = &slice[..bot];
        let end = {
            if top == slice.len() {
                ""
            } else {
                &slice[top..]
            }
        };
        self.word = format!("{}{}", start, end);

        self.cursor = bot;
    }

    pub fn left(&mut self) {
        self.cursor = self.word.floor_char_boundary(self.cursor.saturating_sub(1));
    }

    pub fn right(&mut self) {
        if self.cursor == self.word.len() {
            return;
        }
        self.cursor = self.word.ceil_char_boundary(self.cursor + 1);
    }

    pub fn clear(&mut self) {
        self.word.clear();
        self.cursor = 0;
    }

    pub fn set(&mut self, word: String) {
        self.word = word;
        self.cursor = self.word.len();
    }

    pub fn display_cursor(&self) -> usize {
        let above = self.word.ceil_char_boundary(self.cursor);
        self.word[..above].chars().count()
    }
}

