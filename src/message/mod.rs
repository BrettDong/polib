//! `Message` struct and associated features.

mod flags;
mod key;
mod view;

pub use flags::MessageFlags;
pub(crate) use key::MessageKey;
pub use view::{MessageView, MutableMessageView, SingularPluralMismatchError};

use std::str::FromStr;

/// Represents a single message entry.
#[derive(Debug)]
pub struct Message {
    /// Developer comments of the message.
    pub comments: String,
    /// Source code location of the message.
    pub source: String,
    /// Flags of the message.
    pub flags: MessageFlags,
    /// `msgctxt` of the message.
    pub msgctxt: String,
    /// `msgid` of the message.
    pub msgid: String,
    /// `msgid_plural` of the plural message.
    pub msgid_plural: String,
    /// `msgstr` of the singular message.
    pub msgstr: String,
    /// vector of all plural forms of translation.
    pub msgstr_plural: Vec<String>,
    /// Whether the message is plural
    pub is_plural: bool,
}

impl Message {
    /// Create a new singular message.
    pub fn new_singular(
        comments: &str,
        source: &str,
        flags: &str,
        msgctxt: &str,
        msgid: &str,
        msgstr: &str,
    ) -> Self {
        Message {
            comments: comments.to_string(),
            source: source.to_string(),
            flags: MessageFlags::from_str(flags).unwrap(),
            msgctxt: msgctxt.to_string(),
            msgid: msgid.to_string(),
            msgstr: msgstr.to_string(),
            msgid_plural: String::new(),
            msgstr_plural: vec![],
            is_plural: false,
        }
    }

    /// Create a new singular message by moving from existing string buffers.
    pub fn move_singular_from(
        comments: String,
        source: String,
        flags: String,
        msgctxt: String,
        msgid: String,
        msgstr: String,
    ) -> Self {
        Message {
            comments,
            source,
            flags: MessageFlags::from_str(&flags).unwrap(),
            msgctxt,
            msgid,
            msgstr,
            msgid_plural: String::new(),
            msgstr_plural: vec![],
            is_plural: false,
        }
    }

    /// Create a new plural message.
    pub fn new_plural(
        comments: &str,
        source: &str,
        flags: &str,
        msgctxt: &str,
        msgid: &str,
        msgid_plural: &str,
        msgstr_plural: Vec<String>,
    ) -> Self {
        Message {
            comments: comments.to_string(),
            source: source.to_string(),
            flags: MessageFlags::from_str(flags).unwrap(),
            msgctxt: msgctxt.to_string(),
            msgid: msgid.to_string(),
            msgid_plural: msgid_plural.to_string(),
            msgstr: String::new(),
            msgstr_plural: msgstr_plural.to_vec(),
            is_plural: true,
        }
    }

    /// Create a new plural message by moving from existing string buffers.
    pub fn move_plural_from(
        comments: String,
        source: String,
        flags: String,
        msgctxt: String,
        msgid: String,
        msgid_plural: String,
        msgstr_plural: Vec<String>,
    ) -> Self {
        Message {
            comments,
            source,
            flags: MessageFlags::from_str(&flags).unwrap(),
            msgctxt,
            msgid,
            msgid_plural,
            msgstr: String::new(),
            msgstr_plural,
            is_plural: true,
        }
    }
}
