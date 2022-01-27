//! Parse PO files.

extern crate linereader;
use super::escape::unescape;
use crate::catalog::{Catalog, InvalidCatalogError};
use crate::message::*;
use crate::metadata::CatalogMetadata;
use linereader::LineReader;
use std::error::Error;
use std::path::Path;

fn append_str(buf: &mut String, content: &str) {
    buf.push_str(content);
}

fn append_new_line_str(buf: &mut String, content: &str) {
    if !buf.is_empty() && !buf.ends_with('\n') {
        buf.push('\n');
    }
    buf.push_str(content);
}

/// PO file parse options.
pub struct POParseOptions {
    /// If true, only parse msgctxt, msgid and msgstr.
    pub message_body_only: bool,
    /// If true, skip parsing untranslated messages.
    pub translated_only: bool,
    /// If false, skip maintaining internal hash map.
    pub hash_map: bool,
}

impl POParseOptions {
    /// Creates a default POParseOptions
    pub fn new() -> Self {
        POParseOptions {
            message_body_only: false,
            translated_only: false,
            hash_map: true,
        }
    }
}

impl Default for POParseOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse the PO file and returns a catalog on success.
pub fn parse(path: &Path, options: &POParseOptions) -> Result<Catalog, Box<dyn Error>> {
    let file = std::fs::File::open(path)?;
    let mut metadata_parsed = false;
    let mut idle_buf = String::new();
    let mut cur_str_buf = &mut idle_buf;
    let mut catalog = Catalog::new();
    catalog.messages.push(Message::new());
    let mut clean = true;

    let mut reader = LineReader::new(file);
    while let Some(line) = reader.next_line() {
        if line.is_err() {
            return Err(Box::new(line.err().unwrap()));
        }
        let mut line = unsafe { std::str::from_utf8_unchecked(line.unwrap()) };
        if line.ends_with('\n') {
            line = &line[0..line.len() - 1];
        }
        if line.ends_with('\r') {
            line = &line[0..line.len() - 1];
        }
        if line.is_empty() {
            cur_str_buf = &mut idle_buf;
            if !clean {
                if !metadata_parsed {
                    let metadata_message = catalog.messages.remove(0);
                    if metadata_message.get_msgid().unwrap().is_empty()
                        && metadata_message.is_translated()
                    {
                        catalog.metadata =
                            CatalogMetadata::parse(&unescape(&metadata_message.msgstr).unwrap());
                        metadata_parsed = true;
                    } else {
                        return Err(Box::new(InvalidCatalogError(String::from(
                            "Metadata does not exist or is ill formed",
                        ))));
                    }
                } else if !options.translated_only
                    || catalog.messages.last().unwrap().is_translated()
                {
                    let mut message = catalog.messages.last_mut().unwrap();
                    if message.is_singular() {
                        message.msgctxt = unescape(&message.msgctxt)?;
                        message.msgid = unescape(&message.msgid)?;
                        message.msgstr = unescape(&message.msgstr)?;
                    } else {
                        message.msgctxt = unescape(&message.msgctxt)?;
                        message.msgid = unescape(&message.msgid)?;
                        message.msgid_plural = unescape(&message.msgid_plural)?;
                        message
                            .msgstr_plural
                            .iter_mut()
                            .for_each(|plural| *plural = unescape(plural).unwrap());
                    }
                } else {
                    catalog.messages.remove(catalog.messages.len() - 1);
                }
                catalog.messages.push(Message::new());
                clean = true;
            }
        } else if line.starts_with("#.") && !options.message_body_only {
            cur_str_buf = &mut catalog.messages.last_mut().unwrap().comments;
            append_new_line_str(cur_str_buf, &line[3..]);
            clean = false;
        } else if line.starts_with("#:") && !options.message_body_only {
            cur_str_buf = &mut catalog.messages.last_mut().unwrap().source;
            append_new_line_str(cur_str_buf, &line[3..]);
            clean = false;
        } else if line.starts_with("#,") && !options.message_body_only {
            cur_str_buf = &mut catalog.messages.last_mut().unwrap().flags;
            append_new_line_str(cur_str_buf, &line[3..]);
            clean = false;
        } else if line.starts_with("msgctxt ") {
            cur_str_buf = &mut catalog.messages.last_mut().unwrap().msgctxt;
            let prefix_len = "msgctxt ".len();
            let trimmed = &line[prefix_len..];
            append_str(cur_str_buf, &trimmed[1..trimmed.len() - 1]);
            clean = false;
        } else if line.starts_with("msgid ") {
            cur_str_buf = &mut catalog.messages.last_mut().unwrap().msgid;
            let prefix_len = "msgid ".len();
            let trimmed = &line[prefix_len..];
            append_str(cur_str_buf, &trimmed[1..trimmed.len() - 1]);
            clean = false;
        } else if line.starts_with("msgid_plural ") {
            catalog.messages.last_mut().unwrap().is_plural = true;
            catalog
                .messages
                .last_mut()
                .unwrap()
                .msgstr_plural
                .resize(catalog.metadata.plural_rules.nplurals, String::new());
            cur_str_buf = &mut catalog.messages.last_mut().unwrap().msgid_plural;
            let prefix_len = "msgid_plural ".len();
            let trimmed = &line[prefix_len..];
            append_str(cur_str_buf, &trimmed[1..trimmed.len() - 1]);
            clean = false;
        } else if line.starts_with("msgstr ") {
            cur_str_buf = &mut catalog.messages.last_mut().unwrap().msgstr;
            let prefix_len = "msgstr ".len();
            let trimmed = &line[prefix_len..];
            append_str(cur_str_buf, &trimmed[1..trimmed.len() - 1]);
            clean = false;
        } else if line.starts_with("msgstr[") {
            let index = line.chars().nth(7).unwrap().to_digit(10).unwrap() as usize;
            cur_str_buf = &mut catalog.messages.last_mut().unwrap().msgstr_plural[index];
            let trimmed = &line[10..];
            append_str(cur_str_buf, &trimmed[1..trimmed.len() - 1]);
            clean = false;
        } else if line.starts_with('"') {
            append_str(cur_str_buf, &line[1..line.len() - 1]);
        }
    }

    if options.hash_map {
        for (i, message) in catalog.messages.iter().enumerate() {
            catalog.map.insert(message.internal_key(), i);
        }
    }

    Ok(catalog)
}
