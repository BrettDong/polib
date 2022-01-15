//! Parse PO files.

use super::escape::unescape;
use crate::catalog::{Catalog, InvalidCatalogError};
use crate::message::*;
use crate::metadata::CatalogMetadata;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::path::Path;

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
    fn new() -> Self {
        POParserState {
            cur_comments: String::new(),
            cur_source: String::new(),
            cur_flags: String::new(),
            cur_msgctxt: String::new(),
            cur_msgid: String::new(),
            cur_msgid_plural: String::new(),
            cur_msgstr: String::new(),
            cur_msgstr_plural: vec![String::new(); 10],
            dirty: false,
        }
    }

    fn set_nplurals(&mut self, nplurals: usize) {
        self.cur_msgstr_plural.clear();
        self.cur_msgstr_plural.resize(nplurals, String::new());
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
                unescape(&self.cur_msgctxt).unwrap().as_str(),
                unescape(&self.cur_msgid).unwrap().as_str(),
                unescape(&self.cur_msgstr).unwrap().as_str(),
            );
            self.reset_singular();
        } else {
            let escaped_plural_translations = self
                .cur_msgstr_plural
                .iter()
                .map(|s| unescape(s).unwrap())
                .collect();
            result = Message::new_plural(
                &self.cur_comments,
                &self.cur_source,
                &self.cur_flags,
                unescape(&self.cur_msgctxt).unwrap().as_str(),
                unescape(&self.cur_msgid).unwrap().as_str(),
                unescape(&self.cur_msgid_plural).unwrap().as_str(),
                escaped_plural_translations,
            );
            self.reset_plural();
        }
        result
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

/// Parse the PO file and returns a catalog on success.
pub fn parse(path: &Path) -> Result<Catalog, Box<dyn Error>> {
    let file = std::fs::File::open(path)?;
    let mut metadata_parsed = false;
    let mut state = POParserState::new();
    let mut idle_buf = String::new();
    let mut cur_str_buf = &mut state.cur_msgid;
    let mut catalog = Catalog::new();

    for line in BufReader::new(file).lines() {
        let line = line?;
        if line.is_empty() {
            cur_str_buf = &mut idle_buf;
            if state.dirty {
                let message = state.save_current_message();
                if !metadata_parsed {
                    if message.get_msgid().unwrap().is_empty()
                        && !message.get_msgstr().unwrap().is_empty()
                    {
                        catalog.metadata = CatalogMetadata::parse(message.get_msgstr().unwrap());
                        state.set_nplurals(catalog.metadata.plural_rules.nplurals);
                        metadata_parsed = true;
                    } else {
                        return Err(Box::new(InvalidCatalogError(String::from(
                            "Metadata does not exist",
                        ))));
                    }
                } else {
                    catalog.add_message(message);
                }
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
        } else if line.starts_with("msgctxt ") {
            cur_str_buf = &mut state.cur_msgctxt;
            let prefix_len = "msgctxt ".len();
            let trimmed = &line[prefix_len..];
            append_str(cur_str_buf, &trimmed[1..trimmed.len() - 1]);
            state.dirty = true;
        } else if line.starts_with("msgid ") {
            cur_str_buf = &mut state.cur_msgid;
            let prefix_len = "msgid ".len();
            let trimmed = &line[prefix_len..];
            append_str(cur_str_buf, &trimmed[1..trimmed.len() - 1]);
            state.dirty = true;
        } else if line.starts_with("msgid_plural ") {
            cur_str_buf = &mut state.cur_msgid_plural;
            let prefix_len = "msgid_plural ".len();
            let trimmed = &line[prefix_len..];
            append_str(cur_str_buf, &trimmed[1..trimmed.len() - 1]);
            state.dirty = true;
        } else if line.starts_with("msgstr ") {
            cur_str_buf = &mut state.cur_msgstr;
            let prefix_len = "msgstr ".len();
            let trimmed = &line[prefix_len..];
            append_str(cur_str_buf, &trimmed[1..trimmed.len() - 1]);
            state.dirty = true;
        } else if line.starts_with("msgstr[") {
            let index = line.chars().nth(7).unwrap().to_digit(10).unwrap() as usize;
            cur_str_buf = &mut state.cur_msgstr_plural[index];
            let trimmed = &line.as_str()[10..];
            append_str(cur_str_buf, &trimmed[1..trimmed.len() - 1]);
            state.dirty = true;
        } else if line.starts_with('"') {
            append_str(cur_str_buf, &line[1..line.len() - 1]);
            state.dirty = true;
        }
    }

    if state.dirty {
        let message = state.save_current_message();
        catalog.add_message(message);
    }

    Ok(catalog)
}
