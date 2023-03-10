//! Parse PO files.

extern crate linereader;

use super::escape::unescape;
use crate::catalog::Catalog;
use crate::message::*;
use crate::metadata::{CatalogMetadata, MetadataParseError};
use crate::po_file::escape::UnescapeError;
use linereader::LineReader;
use std::path::Path;

/// PO file parse options.
#[derive(Clone, Copy, Default)]
pub struct POParseOptions {
    /// If true, only parse msgctxt, msgid and msgstr.
    pub message_body_only: bool,
    /// If true, skip parsing untranslated messages.
    pub translated_only: bool,
}

impl POParseOptions {
    /// Creates a default POParseOptions
    pub fn new() -> Self {
        Self::default()
    }
}

/// Error in parsing a PO file
#[derive(Debug)]
pub struct POParseError {
    message: String,
}

impl POParseError {
    fn new(s: &str) -> Self {
        Self {
            message: s.to_string(),
        }
    }
}

impl From<std::io::Error> for POParseError {
    fn from(value: std::io::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<MetadataParseError> for POParseError {
    fn from(value: MetadataParseError) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<UnescapeError> for POParseError {
    fn from(value: UnescapeError) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl std::fmt::Display for POParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PO parse error: {}", self.message)
    }
}

impl std::error::Error for POParseError {}

#[derive(Clone, Copy)]
enum POMessageField {
    None,
    Comments,
    Source,
    Flags,
    Context,
    ID,
    IDPlural,
    Translated,
    TranslatedPlural(usize),
}

#[derive(Default, Debug)]
struct POMessage {
    comments: String,
    source: String,
    flags: String,
    msgctxt: String,
    msgid: String,
    msgid_plural: String,
    msgstr: String,
    msgstr_plural: Vec<String>,
}

struct POParserState {
    dirty: bool,
    metadata_parsed: bool,
    options: POParseOptions,
    current_message: POMessage,
    current_field: POMessageField,
    catalog: Catalog,
}

impl POMessage {
    pub fn is_plural(&self) -> bool {
        !self.msgid_plural.is_empty()
    }

    pub fn is_translated(&self) -> bool {
        if self.is_plural() {
            !(self.msgstr_plural.is_empty() || self.msgstr_plural[0].is_empty())
        } else {
            !self.msgstr.is_empty()
        }
    }
}

impl Default for POParserState {
    fn default() -> Self {
        Self::new(&POParseOptions::new())
    }
}

impl POParserState {
    pub fn new(options: &POParseOptions) -> Self {
        POParserState {
            dirty: false,
            metadata_parsed: false,
            options: *options,
            current_message: POMessage::default(),
            current_field: POMessageField::None,
            catalog: Catalog::new(),
        }
    }

    fn get_field(&mut self) -> &mut String {
        let message = &mut self.current_message;
        match self.current_field {
            POMessageField::Comments => &mut message.comments,
            POMessageField::Source => &mut message.source,
            POMessageField::Flags => &mut message.flags,
            POMessageField::Context => &mut message.msgctxt,
            POMessageField::ID => &mut message.msgid,
            POMessageField::IDPlural => &mut message.msgid_plural,
            POMessageField::Translated => &mut message.msgstr,
            POMessageField::TranslatedPlural(idx) => {
                while message.msgstr_plural.len() <= idx {
                    message.msgstr_plural.push(String::new());
                }
                &mut message.msgstr_plural[idx]
            }
            _ => panic!(),
        }
    }

    fn fill_field(&mut self, data: &str) {
        self.get_field().push_str(data)
    }

    fn fill_field_with_newline(&mut self, data: &str) {
        let field = self.get_field();
        if !field.is_empty() && !field.ends_with('\n') {
            field.push('\n');
        }
        field.push_str(data)
    }

    fn save_message(&mut self) -> Result<(), POParseError> {
        let mut po_message = std::mem::take(&mut self.current_message);
        if !self.metadata_parsed {
            if po_message.msgid.is_empty() && !po_message.msgstr.is_empty() {
                let unescaped = unescape(&po_message.msgstr)?;
                self.catalog.metadata = CatalogMetadata::parse(&unescaped)?;
                self.metadata_parsed = true;
            } else {
                return Err(POParseError::new(
                    "Metadata does not exist or is ill-formed",
                ));
            }
        } else if po_message.is_translated() || !self.options.translated_only {
            if po_message.is_plural() {
                for plural_form in po_message.msgstr_plural.iter_mut() {
                    *plural_form = unescape(plural_form)?;
                }
                self.catalog.append_or_update(Message::move_plural_from(
                    po_message.comments,
                    po_message.source,
                    po_message.flags,
                    unescape(&po_message.msgctxt)?,
                    unescape(&po_message.msgid)?,
                    unescape(&po_message.msgid_plural)?,
                    po_message.msgstr_plural,
                ));
            } else {
                self.catalog.append_or_update(Message::move_singular_from(
                    po_message.comments,
                    po_message.source,
                    po_message.flags,
                    unescape(&po_message.msgctxt)?,
                    unescape(&po_message.msgid)?,
                    unescape(&po_message.msgstr)?,
                ));
            }
        }
        Ok(())
    }

    pub fn consume_line(&mut self, line: &str) -> Result<(), POParseError> {
        static HEADER_FIELDS: [(&str, POMessageField); 3] = [
            ("#. ", POMessageField::Comments),
            ("#: ", POMessageField::Source),
            ("#, ", POMessageField::Flags),
        ];
        static CONTENT_FIELDS: [(&str, POMessageField); 14] = [
            ("msgctxt ", POMessageField::Context),
            ("msgid ", POMessageField::ID),
            ("msgid_plural ", POMessageField::IDPlural),
            ("msgstr ", POMessageField::Translated),
            ("msgstr[0] ", POMessageField::TranslatedPlural(0)),
            ("msgstr[1] ", POMessageField::TranslatedPlural(1)),
            ("msgstr[2] ", POMessageField::TranslatedPlural(2)),
            ("msgstr[3] ", POMessageField::TranslatedPlural(3)),
            ("msgstr[4] ", POMessageField::TranslatedPlural(4)),
            ("msgstr[5] ", POMessageField::TranslatedPlural(5)),
            ("msgstr[6] ", POMessageField::TranslatedPlural(6)),
            ("msgstr[7] ", POMessageField::TranslatedPlural(7)),
            ("msgstr[8] ", POMessageField::TranslatedPlural(8)),
            ("msgstr[9] ", POMessageField::TranslatedPlural(9)),
        ];

        if line.is_empty() {
            if self.dirty {
                self.save_message()?;
                self.dirty = false;
            }
        } else if line.starts_with('#') {
            if !self.options.message_body_only {
                for (prefix, field) in &HEADER_FIELDS {
                    if line.starts_with(*prefix) {
                        self.current_field = *field;
                        self.fill_field_with_newline(&line[prefix.len()..]);
                        self.dirty = true;
                        break;
                    }
                }
            }
        } else if line.starts_with('m') {
            for (prefix, field) in &CONTENT_FIELDS {
                if line.starts_with(*prefix) {
                    self.current_field = *field;
                    let trimmed = &line[prefix.len()..];
                    self.fill_field(&trimmed[1..trimmed.len() - 1]);
                    self.dirty = true;
                }
            }
        } else if line.starts_with('"') {
            self.fill_field(&line[1..line.len() - 1]);
            self.dirty = true;
        }

        Ok(())
    }
}

/// Parse the PO file and returns a catalog on success.
pub fn parse(path: &Path, options: &POParseOptions) -> Result<Catalog, POParseError> {
    let file = std::fs::File::open(path)?;
    let mut parser = POParserState::new(options);
    let mut reader = LineReader::new(file);
    while let Some(line) = reader.next_line() {
        let line = line?;
        let mut line = unsafe { std::str::from_utf8_unchecked(line) };
        if line.ends_with('\n') {
            line = &line[0..line.len() - 1];
        }
        if line.ends_with('\r') {
            line = &line[0..line.len() - 1];
        }
        parser.consume_line(line)?;
    }
    parser.consume_line("")?;
    Ok(parser.catalog)
}
