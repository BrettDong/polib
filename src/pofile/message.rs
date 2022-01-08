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
        msgstr_plural: &[String],
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
                msgstr_plural: msgstr_plural.to_vec(),
            }),
        }
    }

    pub fn internal_key(&self) -> String {
        gen_internal_key(
            self.get_msgctxt().unwrap_or(&String::new()),
            self.get_msgid(),
        )
    }

    pub fn is_plural(&self) -> bool {
        match &self.body {
            PluralizableMessage::Singular(_) => false,
            PluralizableMessage::Plural(_) => true,
        }
    }

    pub fn get_msgctxt(&self) -> Option<&String> {
        match &self.msgctxt {
            Some(ctxt) => Some(ctxt),
            None => None,
        }
    }

    pub fn get_msgid(&self) -> &String {
        match &self.body {
            PluralizableMessage::Singular(singular) => &singular.msgid,
            PluralizableMessage::Plural(plural) => &plural.msgid,
        }
    }

    pub fn get_msgstr(&self) -> Result<&String, SingularPluralMismatchError> {
        match &self.body {
            PluralizableMessage::Singular(singular) => Ok(&singular.msgstr),
            PluralizableMessage::Plural(_) => Err(SingularPluralMismatchError),
        }
    }

    pub fn get_msgstr_plural(&self) -> Result<&Vec<String>, SingularPluralMismatchError> {
        match &self.body {
            PluralizableMessage::Singular(_) => Err(SingularPluralMismatchError),
            PluralizableMessage::Plural(plural) => Ok(&plural.msgstr_plural),
        }
    }
}

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
        use crate::pofile::Message;
        let message = Message::new_singular("", "", "", "", "ID", "STR");
        assert_eq!("ID", message.internal_key());
    }

    #[test]
    fn test_internal_key_with_ctxt() {
        use crate::pofile::Message;
        let message = Message::new_singular("", "", "", "CTXT", "ID", "STR");
        assert_eq!("CTXT\u{0004}ID", message.internal_key());
    }
}
