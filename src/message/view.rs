//! Defines `MessageView` trait.

use std::borrow::{Borrow, ToOwned};
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

/// Immutable view of a `Message`.
pub trait MessageView {
    /// Is this message singular?
    fn is_singular(&self) -> bool;

    /// Is this message plural?
    fn is_plural(&self) -> bool;

    /// Is this message translated?
    fn is_translated(&self) -> bool;

    /// Is this message fuzzy?
    fn is_fuzzy(&self) -> bool;

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

/// Mutable view of a `Message`.
pub trait MessageMutView: MessageView {
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

/// Mutable view of a `Message` that is part of a catalog.
pub trait CatalogMessageMutView: MessageMutView {
    /// Delete this message from the catalog.
    fn delete(&mut self);

    /// Detach this message from the catalog and get the message object back.
    fn detach(&mut self) -> Message;
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

    fn is_fuzzy(&self) -> bool {
        self.flags.is_fuzzy()
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

impl MessageMutView for Message {
    fn comments_mut(&mut self) -> &mut String {
        &mut self.comments
    }

    fn source_mut(&mut self) -> &mut String {
        &mut self.source
    }

    fn flags_mut(&mut self) -> &mut MessageFlags {
        &mut self.flags
    }

    fn set_msgctxt(&mut self, msgctxt: String) {
        self.msgctxt = msgctxt
    }

    fn set_msgid(&mut self, msgid: String) {
        self.msgid = msgid
    }

    fn set_msgid_plural(
        &mut self,
        msgid_plural: String,
    ) -> Result<(), SingularPluralMismatchError> {
        if self.is_plural() {
            self.msgid_plural = msgid_plural;
            Ok(())
        } else {
            Err(SingularPluralMismatchError)
        }
    }

    fn set_msgstr(&mut self, msgstr: String) -> Result<(), SingularPluralMismatchError> {
        if self.is_singular() {
            self.msgstr = msgstr;
            Ok(())
        } else {
            Err(SingularPluralMismatchError)
        }
    }

    fn msgstr_mut(&mut self) -> Result<&mut String, SingularPluralMismatchError> {
        if self.is_singular() {
            Ok(&mut self.msgstr)
        } else {
            Err(SingularPluralMismatchError)
        }
    }

    fn msgstr_plural_mut(&mut self) -> Result<&mut Vec<String>, SingularPluralMismatchError> {
        if self.is_plural() {
            Ok(&mut self.msgstr_plural)
        } else {
            Err(SingularPluralMismatchError)
        }
    }
}

impl<'a> Borrow<dyn MessageView + 'a> for Message {
    fn borrow(&self) -> &(dyn MessageView + 'a) {
        self
    }
}

impl ToOwned for dyn MessageView {
    type Owned = Message;

    fn to_owned(&self) -> Self::Owned {
        if self.is_singular() {
            Self::Owned {
                comments: self.comments().to_string(),
                source: self.source().to_string(),
                flags: self.flags().clone(),
                msgctxt: self.msgctxt().to_string(),
                msgid: self.msgid().to_string(),
                msgid_plural: String::default(),
                msgstr: self.msgstr().unwrap().to_string(),
                msgstr_plural: vec![],
                is_plural: false,
            }
        } else {
            Self::Owned {
                comments: self.comments().to_string(),
                source: self.source().to_string(),
                flags: self.flags().clone(),
                msgctxt: self.msgctxt().to_string(),
                msgid: self.msgid().to_string(),
                msgid_plural: self.msgid_plural().unwrap().to_string(),
                msgstr: String::default(),
                msgstr_plural: self.msgstr_plural().unwrap().to_owned(),
                is_plural: true,
            }
        }
    }
}
