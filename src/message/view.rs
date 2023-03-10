//! Defines `MessageView` trait.

use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::message::{Message, MessageFlags};

/// Error type when trying to access a field that is not applicable to the plural type of the message.
#[derive(Debug)]
pub struct SingularPluralMismatchError;

impl Display for SingularPluralMismatchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "singular/plural type mismatch")
    }
}

impl Error for SingularPluralMismatchError {}

/// Immutable view of a `Message`
pub trait MessageView {
    /// Is this message singular?
    fn is_singular(&self) -> bool;

    /// Is this message plural?
    fn is_plural(&self) -> bool;

    /// Is this message translated?
    fn is_translated(&self) -> bool;

    /// Get comments field of the message.
    fn comments(&self) -> &str;

    /// Get source code location field of the message.
    fn source(&self) -> &str;

    /// Get flags field of the message.
    fn flags(&self) -> &MessageFlags;

    /// Get context field of the message.
    fn msgctxt(&self) -> &str;

    /// Get msgid field of the message.
    fn msgid(&self) -> &str;

    /// Get msgid_plural field of the message, or error if this is not a plural message.
    fn msgid_plural(&self) -> Result<&str, SingularPluralMismatchError>;

    /// Get msgstr field of the message, or error if this is not a singular message.
    fn msgstr(&self) -> Result<&str, SingularPluralMismatchError>;

    /// Get msgstr_plural field of the message, or error if this is not a plural message.
    fn msgstr_plural(&self) -> Result<&Vec<String>, SingularPluralMismatchError>;
}

/// Mutable view of a `Message`
pub trait MutableMessageView: MessageView {
    /// Get a mutable reference to the comments field of the message.
    fn comments_mut(&mut self) -> &mut String;

    /// Get a mutable reference to the source code location field of the message.
    fn source_mut(&mut self) -> &mut String;

    /// Get a mutable reference to the flags field of the message.
    fn flags_mut(&mut self) -> &mut MessageFlags;

    /// Set the context field of the message.
    fn set_msgctxt(&mut self, msgctxt: String);

    /// Set the msgid field of the message.
    fn set_msgid(&mut self, msgid: String);

    /// Set the msgid_plural field of the message, or error if this is not a plural message.
    fn set_msgid_plural(&mut self, msgid_plural: String)
        -> Result<(), SingularPluralMismatchError>;

    /// Set the msgstr field of the message, or error if this is not a singular message.
    fn set_msgstr(&mut self, msgstr: String) -> Result<(), SingularPluralMismatchError>;

    /// Get a mutable reference to the msgstr field of the message, or error if this is not a singular message.
    fn msgstr_mut(&mut self) -> Result<&mut String, SingularPluralMismatchError>;

    /// Get a mutable reference to the msgstr_plural field of the message, or error if this is not a plural message.
    fn msgstr_plural_mut(&mut self) -> Result<&mut Vec<String>, SingularPluralMismatchError>;
}

impl MessageView for Message {
    fn is_singular(&self) -> bool {
        !self.is_plural
    }

    fn is_plural(&self) -> bool {
        self.is_plural
    }

    fn is_translated(&self) -> bool {
        if self.is_plural {
            self.msgstr_plural.iter().all(|x| !x.is_empty())
        } else {
            !self.msgstr.is_empty()
        }
    }

    fn comments(&self) -> &str {
        &self.comments
    }

    fn source(&self) -> &str {
        &self.source
    }

    fn flags(&self) -> &MessageFlags {
        &self.flags
    }

    fn msgctxt(&self) -> &str {
        &self.msgctxt
    }

    fn msgid(&self) -> &str {
        &self.msgid
    }

    fn msgid_plural(&self) -> Result<&str, SingularPluralMismatchError> {
        if self.is_plural {
            Ok(&self.msgid_plural)
        } else {
            Err(SingularPluralMismatchError)
        }
    }

    fn msgstr(&self) -> Result<&str, SingularPluralMismatchError> {
        if self.is_plural {
            Err(SingularPluralMismatchError)
        } else {
            Ok(&self.msgstr)
        }
    }

    fn msgstr_plural(&self) -> Result<&Vec<String>, SingularPluralMismatchError> {
        if self.is_plural {
            Ok(&self.msgstr_plural)
        } else {
            Err(SingularPluralMismatchError)
        }
    }
}
