use std::borrow::Cow;
use std::ops::Range;
use druid::{Data};
use druid::piet::{TextStorage as PietTextStorage};
use druid::text::{EditableText, StringCursor, TextStorage};

#[derive(Data, Debug, Clone, PartialEq)]
pub struct PasswordText {
    clear: String,
    hidden: String
}

const PASSWORD_CHAR: char = '●';

impl PasswordText {
    pub fn new() -> Self {
        PasswordText {
            clear: String::new(),
            hidden: String::new()
        }
    }

    pub fn value(&self) -> &str {
        &self.clear
    }

    pub fn clear(&mut self) {
        self.clear.replace_range(.., " ");
        self.clear.clear();
        self.hidden.clear();
    }

    fn set_hidden(&mut self) {
        let target_len = self.clear
            .chars()
            .count();

        loop {
            let actual_len = self.hidden
                .chars()
                .count();

            match actual_len {
                x if x < target_len => self.hidden.push(PASSWORD_CHAR),
                x if x > target_len => { self.hidden.pop(); },
                _ => break
            };
        }
    }
}

impl EditableText for PasswordText {
    fn cursor(&self, position: usize) -> Option<StringCursor> {
        <String as EditableText>::cursor(&self.hidden, position)
    }

    fn edit(&mut self, range: Range<usize>, new: impl Into<String>) {
        let new = new.into();
        let range = Range {
            start: range.start / PASSWORD_CHAR.len_utf8(),
            end: range.end / PASSWORD_CHAR.len_utf8()
        };

        self.clear.replace_range(range, &new);
        self.set_hidden();
    }

    fn slice(&self, range: Range<usize>) -> Option<Cow<str>> {
        <String as EditableText>::slice(&self.hidden, range)
    }

    fn len(&self) -> usize {
        <String as EditableText>::len(&self.hidden)
    }

    fn prev_word_offset(&self, offset: usize) -> Option<usize> {
        <String as EditableText>::prev_word_offset(&self.hidden, offset)
    }

    fn next_word_offset(&self, offset: usize) -> Option<usize> {
        <String as EditableText>::next_word_offset(&self.hidden, offset)
    }

    fn prev_grapheme_offset(&self, offset: usize) -> Option<usize> {
        <String as EditableText>::prev_grapheme_offset(&self.hidden, offset)
    }

    fn next_grapheme_offset(&self, offset: usize) -> Option<usize> {
        <String as EditableText>::next_grapheme_offset(&self.hidden, offset)
    }

    fn prev_codepoint_offset(&self, offset: usize) -> Option<usize> {
        <String as EditableText>::prev_codepoint_offset(&self.hidden, offset)
    }

    fn next_codepoint_offset(&self, offset: usize) -> Option<usize> {
        <String as EditableText>::next_codepoint_offset(&self.hidden, offset)
    }

    fn preceding_line_break(&self, offset: usize) -> usize {
        <String as EditableText>::preceding_line_break(&self.hidden, offset)
    }

    fn next_line_break(&self, offset: usize) -> usize {
        <String as EditableText>::next_line_break(&self.hidden, offset)
    }

    fn is_empty(&self) -> bool {
        <String as EditableText>::is_empty(&self.hidden)
    }

    fn from_str(s: &str) -> Self {
        let mut text = PasswordText {
            clear: <String as EditableText>::from_str(s),
            hidden: String::new()
        };
        text.set_hidden();
        text
    }
}

impl PietTextStorage for PasswordText {
    fn as_str(&self) -> &str {
        &self.hidden
    }
}

impl TextStorage for PasswordText {}