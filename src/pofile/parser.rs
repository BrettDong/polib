use std::error::Error;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::pofile::*;

struct POParserState {
    cur_comments: String,
    cur_source: String,
    cur_flags: String,
    cur_msgctxt: String,
    cur_msgid: String,
    cur_msgid_plural: String,
    cur_msgstr: String,
    cur_msgstr_plural: Vec<String>,
    dirty: bool,
}

impl POParserState {
    fn new(num_plural_forms: usize) -> Self {
        POParserState {
            cur_comments: String::new(),
            cur_source: String::new(),
            cur_flags: String::new(),
            cur_msgctxt: String::new(),
            cur_msgid: String::new(),
            cur_msgid_plural: String::new(),
            cur_msgstr: String::new(),
            cur_msgstr_plural: vec![String::new(); num_plural_forms],
            dirty: false,
        }
    }

    fn reset_singular(&mut self) {
        self.cur_comments.clear();
        self.cur_source.clear();
        self.cur_flags.clear();
        self.cur_msgctxt.clear();
        self.cur_msgid.clear();
        self.cur_msgstr.clear();
    }

    fn reset_plural(&mut self) {
        self.cur_comments.clear();
        self.cur_source.clear();
        self.cur_flags.clear();
        self.cur_msgctxt.clear();
        self.cur_msgid.clear();
        self.cur_msgid_plural.clear();
        for form in self.cur_msgstr_plural.iter_mut() {
            form.clear();
        }
    }

    fn save_current_message(&mut self) -> Message {
        let result;
        if self.cur_msgid_plural.is_empty() {
            result = Message::new_singular(
                &self.cur_comments,
                &self.cur_source,
                &self.cur_flags,
                &self.cur_msgctxt,
                &self.cur_msgid,
                &self.cur_msgstr,
            );
            self.reset_singular();
        } else {
            result = Message::new_plural(
                &self.cur_comments,
                &self.cur_source,
                &self.cur_flags,
                &self.cur_msgctxt,
                &self.cur_msgid,
                &self.cur_msgid_plural,
                &self.cur_msgstr_plural,
            );
            self.reset_plural();
        }
        result
    }
}

impl POFile {
    pub fn parse(path: &Path) -> Result<Self, Box<dyn Error>> {
        let file = std::fs::File::open(path)?;
        let num_plural_forms = find_num_plurals(&path)?;
        let mut messages: Vec<Message> = Vec::new();
        let mut state = POParserState::new(num_plural_forms);
        let mut idle_buf = String::new();
        let mut cur_str_buf = &mut state.cur_msgid;

        for line in BufReader::new(file).lines() {
            let line = line?;
            if line.is_empty() {
                cur_str_buf = &mut idle_buf;
                if state.dirty {
                    messages.push(state.save_current_message());
                }
            } else if line.starts_with("#.") {
                cur_str_buf = &mut state.cur_comments;
                append_new_line_str(cur_str_buf, &line.as_str()[3..]);
                state.dirty = true;
            } else if line.starts_with("#:") {
                cur_str_buf = &mut state.cur_source;
                append_new_line_str(cur_str_buf, &line.as_str()[3..]);
                state.dirty = true;
            } else if line.starts_with("#,") {
                cur_str_buf = &mut state.cur_flags;
                append_new_line_str(cur_str_buf, &line.as_str()[3..]);
                state.dirty = true;
            } else if line.starts_with("msgid ") {
                cur_str_buf = &mut state.cur_msgid;
                append_str(
                    cur_str_buf,
                    &line.trim_start_matches("msgid ").trim_matches('"'),
                );
                state.dirty = true;
            } else if line.starts_with("msgid_plural ") {
                cur_str_buf = &mut state.cur_msgid_plural;
                append_str(
                    cur_str_buf,
                    &line.trim_start_matches("msgid_plural ").trim_matches('"'),
                );
                state.dirty = true;
            } else if line.starts_with("msgstr ") {
                cur_str_buf = &mut state.cur_msgstr;
                append_str(
                    cur_str_buf,
                    &line.trim_start_matches("msgstr ").trim_matches('"'),
                );
                state.dirty = true;
            } else if line.starts_with("msgstr[") {
                let index = line.chars().nth(7).unwrap().to_digit(10).unwrap() as usize;
                cur_str_buf = &mut state.cur_msgstr_plural[index];
                append_str(cur_str_buf, &line.as_str()[10..].trim_matches('"'));
                state.dirty = true;
            } else if line.starts_with("\"") {
                append_str(cur_str_buf, &line.trim_matches('"'));
                state.dirty = true;
            }
        }

        if state.dirty {
            messages.push(state.save_current_message());
        }

        Ok(POFile {
            num_plural_forms: num_plural_forms,
            plural_eval: |_| 0,
            messages: messages,
        })
    }
}

fn append_str(buf: &mut String, content: &str) {
    buf.push_str(content);
}

fn append_new_line_str(buf: &mut String, content: &str) {
    if !buf.is_empty() && !buf.ends_with('\n') {
        buf.push('\n');
    }
    buf.push_str(content);
}

fn find_num_plurals(path: &Path) -> Result<usize, std::io::Error> {
    let file = std::fs::File::open(path)?;
    let pattern = "Plural-Forms: nplurals=";
    for line in BufReader::new(file).lines() {
        let line = line?;
        let line = line.as_str();
        match line.find(pattern) {
            Some(index) => {
                return Ok(line
                    .chars()
                    .nth(index + pattern.len())
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as usize);
            }
            None => {
                continue;
            }
        }
    }
    Ok(1)
}
