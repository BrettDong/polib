//! `Message` struct and associated functions.

use std::convert::Infallible;

/// Represents the set of flags in a message
#[derive(Debug)]
pub struct MessageFlags {
    /// Vector of individual flags
    pub entries: Vec<String>,
}

/// Represents a single message entry.
#[derive(Debug)]
pub struct Message {
    /// Developer comments of the message.
    pub comments: String,
    /// Source code location of the message.
    pub source: String,
    /// Flags of the message.
    pub flags: MessageFlags,
    /// (internal) String form of flag list, only used when parsing a PO file
    pub(crate) flags_str: String,
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

impl Default for MessageFlags {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageFlags {
    /// Create an empty set of flags
    pub fn new() -> Self {
        MessageFlags { entries: vec![] }
    }

    /// Parse flags line from PO file
    pub fn parse(flags: &str) -> Self {
        let flags = flags.replace('\n', "");
        let segments = flags.split(',');
        let mut result = Self::new();
        for x in segments {
            if !x.is_empty() {
                result.entries.push(String::from(x.trim()));
            }
        }
        result
    }

    /// Export in PO file format
    pub fn export(&self) -> String {
        if self.is_empty() {
            String::new()
        } else {
            self.entries.join(", ")
        }
    }

    /// Is the set of flags empty?
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Is a flag present?
    pub fn contains(&self, flag: &str) -> bool {
        let flag = flag.to_string();
        self.entries.contains(&flag)
    }

    /// Is fuzzy flag present?
    pub fn is_fuzzy(&self) -> bool {
        self.contains("fuzzy")
    }
}

impl Message {
    pub(crate) fn new() -> Self {
        Message {
            comments: String::new(),
            source: String::new(),
            flags: MessageFlags::new(),
            flags_str: String::new(),
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
            flags: MessageFlags::parse(flags),
            flags_str: String::new(),
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
            flags: MessageFlags::parse(flags),
            flags_str: String::new(),
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
    fn test_flags_from_string() {
        use crate::message::MessageFlags;
        assert_eq!(MessageFlags::parse("").entries, Vec::<String>::new());
        assert_eq!(MessageFlags::parse("fuzzy").entries, vec!["fuzzy"]);
        assert_eq!(
            MessageFlags::parse("c-format, fuzzy").entries,
            vec!["c-format", "fuzzy"]
        );
    }

    #[test]
    fn test_flags_to_string() {
        use crate::message::MessageFlags;
        assert_eq!(MessageFlags { entries: vec![] }.export(), "");
        assert_eq!(
            MessageFlags {
                entries: vec![String::from("fuzzy")]
            }
            .export(),
            "fuzzy"
        );
        assert_eq!(
            MessageFlags {
                entries: vec![String::from("c-format"), String::from("fuzzy")]
            }
            .export(),
            "c-format, fuzzy"
        );
    }

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
