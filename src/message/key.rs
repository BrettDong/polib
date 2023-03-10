//! Defines `MessageKey` struct.

use crate::message::{Message, MessageView};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct MessageKey {
    key: String,
}

impl MessageKey {
    pub(crate) fn gen(msgctxt: Option<&str>, msgid: &str, msgid_plural: Option<&str>) -> Self {
        Self {
            key: match (msgctxt, msgid_plural) {
                (Some(msgctxt), Some(msgid_plural)) => {
                    format!("{}\u{0004}{}\u{0000}{}", msgctxt, msgid, msgid_plural)
                }
                (Some(msgctxt), None) => format!("{}\u{0004}{}", msgctxt, msgid),
                (None, Some(msgid_plural)) => format!("{}\u{0000}{}", msgid, msgid_plural),
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
