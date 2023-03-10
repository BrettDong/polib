//! Defines `MessageKey` struct.

use crate::message::{Message, MessageView};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct MessageKey {
    key: String,
}

impl From<&Message> for MessageKey {
    fn from(m: &Message) -> Self {
        Self {
            key: match (m.msgctxt.is_empty(), m.is_singular()) {
                (true, true) => m.msgid.to_string(),
                (true, false) => format!("{}\u{0000}{}", m.msgid, m.msgid_plural),
                (false, true) => format!("{}\u{0004}{}", m.msgctxt, m.msgid),
                (false, false) => {
                    format!("{}\u{0004}{}\u{0000}{}", m.msgctxt, m.msgid, m.msgid_plural)
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::message::{Message, MessageKey};

    #[test]
    fn test_internal_key_without_ctxt() {
        let message = Message::new_singular("", "", "", "", "ID", "STR");
        assert_eq!("ID", MessageKey::from(&message).key);
    }

    #[test]
    fn test_internal_key_with_ctxt() {
        let message = Message::new_singular("", "", "", "CTXT", "ID", "STR");
        assert_eq!("CTXT\u{0004}ID", MessageKey::from(&message).key);
    }
}
