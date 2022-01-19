//! `Message` struct and associated functions.

/// The body of a singular message.
#[derive(Debug)]
pub struct SingularMessage {
    /// `msgid` of the message.
    pub msgid: String,
    /// `msgstr` of the message.
    pub msgstr: String,
}

/// The body of a plural message.
#[derive(Debug)]
pub struct PluralMessage {
    /// `msgid` of the message.
    pub msgid: String,
    /// `msgid_plural` of the message.
    pub msgid_plural: String,
    /// vector of all plural forms of translation.
    pub msgstr_plural: Vec<String>,
}

/// Stores the body of either a singular message or a plural message.
#[derive(Debug)]
pub enum MessageBody {
    /// The body of a singular message.
    Singular(SingularMessage),
    /// The body of a singular message.
    Plural(PluralMessage),
}

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
    /// body of the message in a enum.
    pub body: MessageBody,
}

/// Error type when trying to access a field not applicable to the singular or
/// plural form of the message.
#[derive(Debug)]
pub struct SingularPluralMismatchError;

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
            flags: flags.to_string(),
            msgctxt: msgctxt.to_string(),
            body: MessageBody::Singular(SingularMessage {
                msgid: msgid.to_string(),
                msgstr: msgstr.to_string(),
            }),
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
            body: MessageBody::Plural(PluralMessage {
                msgid: msgid.to_string(),
                msgid_plural: msgid_plural.to_string(),
                msgstr_plural: msgstr_plural.to_vec(),
            }),
        }
    }

    /// Generate the internal hash map key of the message.
    pub fn internal_key(&self) -> String {
        gen_internal_key(self.get_msgctxt().unwrap(), self.get_msgid().unwrap())
    }

    /// Whether the message is a singular message.
    pub fn is_singular(&self) -> bool {
        match &self.body {
            MessageBody::Singular(_) => true,
            MessageBody::Plural(_) => false,
        }
    }

    /// Whether the message is a plural message.
    pub fn is_plural(&self) -> bool {
        match &self.body {
            MessageBody::Singular(_) => false,
            MessageBody::Plural(_) => true,
        }
    }

    /// Whether the message is translated.
    pub fn is_translated(&self) -> bool {
        match &self.body {
            MessageBody::Singular(SingularMessage { msgstr, .. }) => !msgstr.is_empty(),
            MessageBody::Plural(PluralMessage { msgstr_plural, .. }) => {
                msgstr_plural.iter().all(|x| !x.is_empty())
            }
        }
    }

    /// Get `msgctxt` of the message.
    pub fn get_msgctxt(&self) -> Result<&String, SingularPluralMismatchError> {
        Ok(&self.msgctxt)
    }

    /// Get `msgid` of the message.
    pub fn get_msgid(&self) -> Result<&String, SingularPluralMismatchError> {
        match &self.body {
            MessageBody::Singular(singular) => Ok(&singular.msgid),
            MessageBody::Plural(plural) => Ok(&plural.msgid),
        }
    }

    /// Get `msgid_plural` of the plural message.
    pub fn get_msgid_plural(&self) -> Result<&String, SingularPluralMismatchError> {
        match &self.body {
            MessageBody::Singular(_) => Err(SingularPluralMismatchError),
            MessageBody::Plural(plural) => Ok(&plural.msgid_plural),
        }
    }

    /// Get `msgstr` of the singular message.
    pub fn get_msgstr(&self) -> Result<&String, SingularPluralMismatchError> {
        match &self.body {
            MessageBody::Singular(singular) => Ok(&singular.msgstr),
            MessageBody::Plural(_) => Err(SingularPluralMismatchError),
        }
    }

    /// Get the vector of all `msgstr_plural` of the plural message.
    pub fn get_msgstr_plural(&self) -> Result<&Vec<String>, SingularPluralMismatchError> {
        match &self.body {
            MessageBody::Singular(_) => Err(SingularPluralMismatchError),
            MessageBody::Plural(plural) => Ok(&plural.msgstr_plural),
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
