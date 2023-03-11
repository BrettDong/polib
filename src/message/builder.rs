use crate::message::{Message, MessageFlags};

/// A helper type to build a `Message` conveniently via method chaining.
pub struct MessageBuilder {
    m: Message,
}

impl Message {
    /// Build a new singular message.
    pub fn build_singular() -> MessageBuilder {
        MessageBuilder {
            m: Message {
                is_plural: false,
                ..Message::default()
            },
        }
    }

    /// Build a new plural message.
    pub fn build_plural() -> MessageBuilder {
        MessageBuilder {
            m: Message {
                is_plural: true,
                ..Message::default()
            },
        }
    }
}

impl MessageBuilder {
    /// Set the comments field.
    pub fn with_comments(&mut self, comments: String) -> &mut Self {
        self.m.comments = comments;
        self
    }

    /// Set the source field.
    pub fn with_source(&mut self, source: String) -> &mut Self {
        self.m.source = source;
        self
    }

    /// Set the flags field.
    pub fn with_flags(&mut self, flags: MessageFlags) -> &mut Self {
        self.m.flags = flags;
        self
    }

    /// Set the msgctxt field.
    pub fn with_msgctxt(&mut self, msgctxt: String) -> &mut Self {
        self.m.msgctxt = msgctxt;
        self
    }

    /// Set the msgid field.
    pub fn with_msgid(&mut self, msgid: String) -> &mut Self {
        self.m.msgid = msgid;
        self
    }

    /// Set the msgid_plural field.
    pub fn with_msgid_plural(&mut self, msgid_plural: String) -> &mut Self {
        self.m.msgid_plural = msgid_plural;
        self
    }

    /// Set the msgstr field.
    pub fn with_msgstr(&mut self, msgstr: String) -> &mut Self {
        self.m.msgstr = msgstr;
        self
    }

    /// Set the msgstr_plural field.
    pub fn with_msgstr_plural(&mut self, msgstr_plural: Vec<String>) -> &mut Self {
        self.m.msgstr_plural = msgstr_plural;
        self
    }

    /// Finish building and get the resulting `Message` object.
    /// This builder object should be discarded and not be re-used afterwards.
    pub fn done(&mut self) -> Message {
        std::mem::take(&mut self.m)
    }
}
