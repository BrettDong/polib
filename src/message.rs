//! `Message` struct and associated functions.

use std::convert::Infallible;

/// Represents a single message entry.
#[derive(Debug)]
pub struct Message {
    /// Developer comments of the message.
    pub comments: String,
    /// Source code location of the message.
    pub source: String,
    /// Flags of the message.
    pub flags: String,
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

/// Error type when trying to access a field not applicable to the singular or
/// plural form of the message.
#[derive(Debug)]
pub struct SingularPluralMismatchError;

impl Message {
    pub(crate) fn new() -> Self {
        Message {
            comments: String::new(),
            source: String::new(),
            flags: String::new(),
            msgctxt: String::new(),
            msgid: String::new(),
            msgid_plural: String::new(),
            msgstr: String::new(),
            msgstr_plural: vec![],
            is_plural: false,
        }
    }

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
            flags: flags.to_string(),
            msgctxt: msgctxt.to_string(),
            msgid: msgid.to_string(),
            msgstr: msgstr.to_string(),
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
            flags: flags.to_string(),
            msgctxt: msgctxt.to_string(),
            msgid: msgid.to_string(),
            msgid_plural: msgid_plural.to_string(),
            msgstr: String::new(),
            msgstr_plural: msgstr_plural.to_vec(),
            is_plural: true,
        }
    }

    /// Generate the internal hash map key of the message.
    pub fn internal_key(&self) -> String {
        gen_internal_key(self.get_msgctxt().unwrap(), self.get_msgid().unwrap())
    }

    /// Whether the message is a singular message.
    pub fn is_singular(&self) -> bool {
        !self.is_plural
    }

    /// Whether the message is a plural message.
    pub fn is_plural(&self) -> bool {
        self.is_plural
    }

    /// Whether the message is translated.
    pub fn is_translated(&self) -> bool {
        if self.is_plural {
            self.msgstr_plural.iter().all(|x| !x.is_empty())
        } else {
            !self.msgstr.is_empty()
        }
    }

    /// Get `msgctxt` of the message.
    pub fn get_msgctxt(&self) -> Result<&String, SingularPluralMismatchError> {
        Ok(&self.msgctxt)
    }

    /// Get `msgid` of the message.
    pub fn get_msgid(&self) -> Result<&String, Infallible> {
        Ok(&self.msgid)
    }

    /// Get `msgid_plural` of the plural message.
    pub fn get_msgid_plural(&self) -> Result<&String, SingularPluralMismatchError> {
        if self.is_plural {
            Ok(&self.msgid_plural)
        } else {
            Err(SingularPluralMismatchError)
        }
    }

    /// Get `msgstr` of the singular message.
    pub fn get_msgstr(&self) -> Result<&String, SingularPluralMismatchError> {
        if self.is_plural {
            Err(SingularPluralMismatchError)
        } else {
            Ok(&self.msgstr)
        }
    }

    /// Get the vector of all `msgstr_plural` of the plural message.
    pub fn get_msgstr_plural(&self) -> Result<&Vec<String>, SingularPluralMismatchError> {
        if self.is_plural {
            Ok(&self.msgstr_plural)
        } else {
            Err(SingularPluralMismatchError)
        }
    }
}

/// Generate the key of the message for hash map by concatenating
/// `msgctxt`, `\u{0004}` and `msgid`. This representation is also
/// used in MO binary translation catalog format in GNU gettext.
pub fn gen_internal_key(msgctxt: &str, msgid: &str) -> String {
    if msgctxt.is_empty() {
        msgid.to_string()
    } else {
        format!("{}\u{0004}{}", msgctxt, msgid)
    }
}

mod test {
    #[test]
    fn test_internal_key_without_ctxt() {
        use crate::message::Message;
        let message = Message::new_singular("", "", "", "", "ID", "STR");
        assert_eq!("ID", message.internal_key());
    }

    #[test]
    fn test_internal_key_with_ctxt() {
        use crate::message::Message;
        let message = Message::new_singular("", "", "", "CTXT", "ID", "STR");
        assert_eq!("CTXT\u{0004}ID", message.internal_key());
    }
}
