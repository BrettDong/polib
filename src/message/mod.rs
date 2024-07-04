//! Defines `Message` struct and its view traits.

mod builder;
mod flags;
mod key;
mod view;

pub use builder::MessageBuilder;
pub use flags::MessageFlags;
pub(crate) use key::MessageKey;
pub use view::{CatalogMessageMutView, MessageMutView, MessageView, SingularPluralMismatchError};

/// Represents a single message entry.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Message {
    /// Developer comments of the message.
    pub(crate) comments: String,
    /// Source code location of the message.
    pub(crate) source: String,
    /// Flags of the message.
    pub(crate) flags: MessageFlags,
    /// `msgctxt` of the message.
    pub(crate) msgctxt: String,
    /// `msgid` of the message.
    pub(crate) msgid: String,
    /// `msgid_plural` of the plural message.
    pub(crate) msgid_plural: String,
    /// `msgstr` of the singular message.
    pub(crate) msgstr: String,
    /// vector of all plural forms of translation.
    pub(crate) msgstr_plural: Vec<String>,
    /// Whether the message is plural
    pub(crate) is_plural: bool,
}
