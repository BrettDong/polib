//! Defines `MessageKey` struct.

use crate::message::{Message, MessageView};
use concat_string::concat_string;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct MessageKey {
    key: String,
}

impl MessageKey {
    pub(crate) fn gen(msgctxt: Option<&str>, msgid: &str, msgid_plural: Option<&str>) -> Self {
        Self {
            key: match (msgctxt, msgid_plural) {
                (Some(msgctxt), Some(msgid_plural)) => {
                    concat_string!(msgctxt, "\u{0004}", msgid, "\u{0000}", msgid_plural)
                }
                (Some(msgctxt), None) => {
                    concat_string!(msgctxt, "\u{0004}", msgid)
                }
                (None, Some(msgid_plural)) => {
                    concat_string!(msgid, "\u{0000}", msgid_plural)
                }
                (None, None) => msgid.to_string(),
            },
        }
    }
}

impl From<&Message> for MessageKey {
    fn from(m: &Message) -> Self {
        match (!m.msgctxt.is_empty(), m.is_plural()) {
            (true, true) => Self::gen(Some(&m.msgctxt), &m.msgid, Some(&m.msgid_plural)),
            (true, false) => Self::gen(Some(&m.msgctxt), &m.msgid, None),
            (false, true) => Self::gen(None, &m.msgid, Some(&m.msgid_plural)),
            (false, false) => Self::gen(None, &m.msgid, None),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::message::{Message, MessageKey};

    #[test]
    fn test_singular_message_key_without_ctxt() {
        let message = Message::build_singular()
            .with_msgid(String::from("ID"))
            .done();
        assert_eq!("ID", MessageKey::from(&message).key);
    }

    #[test]
    fn test_singular_message_key_with_ctxt() {
        let message = Message::build_singular()
            .with_msgctxt(String::from("CTXT"))
            .with_msgid(String::from("ID"))
            .done();
        assert_eq!("CTXT\u{0004}ID", MessageKey::from(&message).key);
    }
}
