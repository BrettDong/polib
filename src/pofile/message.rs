use crate::pofile::*;

impl Message {
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
            msgctxt: if msgctxt.is_empty() {
                None
            } else {
                Some(msgctxt.to_string())
            },
            body: PluralizableMessage::Singular(SingularMessage {
                msgid: msgid.to_string(),
                msgstr: msgstr.to_string(),
            }),
        }
    }

    pub fn new_plural(
        comments: &str,
        source: &str,
        flags: &str,
        msgctxt: &str,
        msgid: &str,
        msgid_plural: &str,
        msgstr_plural: &Vec<String>,
    ) -> Self {
        Message {
            comments: comments.to_string(),
            source: source.to_string(),
            flags: flags.to_string(),
            msgctxt: if msgctxt.is_empty() {
                None
            } else {
                Some(msgctxt.to_string())
            },
            body: PluralizableMessage::Plural(PluralMessage {
                msgid: msgid.to_string(),
                msgid_plural: msgid_plural.to_string(),
                msgstr_plural: msgstr_plural.to_vec()
            }),
        }
    }
}
