#[derive(Debug)]
pub struct SingularMessage {
    pub msgid: String,
    pub msgstr: String,
}

#[derive(Debug)]
pub struct PluralMessage {
    pub msgid: String,
    pub msgid_plural: String,
    pub msgstr_plural: Vec<String>,
}

#[derive(Debug)]
pub enum MessageBody {
    Singular(SingularMessage),
    Plural(PluralMessage),
}

#[derive(Debug)]
pub struct Message {
    pub comments: String,
    pub source: String,
    pub flags: String,
    pub msgctxt: String,
    pub body: MessageBody,
}

#[derive(Debug)]
pub struct SingularPluralMismatchError;

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
            msgctxt: msgctxt.to_string(),
            body: MessageBody::Singular(SingularMessage {
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
            msgctxt: msgctxt.to_string(),
            body: MessageBody::Plural(PluralMessage {
                msgid: msgid.to_string(),
                msgid_plural: msgid_plural.to_string(),
                msgstr_plural: msgstr_plural.to_vec(),
            }),
        }
    }

    pub fn internal_key(&self) -> String {
        gen_internal_key(self.get_msgctxt().unwrap(), self.get_msgid().unwrap())
    }

    pub fn is_singular(&self) -> bool {
        match &self.body {
            MessageBody::Singular(_) => true,
            MessageBody::Plural(_) => false,
        }
    }

    pub fn is_plural(&self) -> bool {
        match &self.body {
            MessageBody::Singular(_) => false,
            MessageBody::Plural(_) => true,
        }
    }

    pub fn get_msgctxt(&self) -> Result<&String, ()> {
        Ok(&self.msgctxt)
    }

    pub fn get_msgid(&self) -> Result<&String, ()> {
        match &self.body {
            MessageBody::Singular(singular) => Ok(&singular.msgid),
            MessageBody::Plural(plural) => Ok(&plural.msgid),
        }
    }

    pub fn get_msgid_plural(&self) -> Result<&String, SingularPluralMismatchError> {
        match &self.body {
            MessageBody::Singular(_) => Err(SingularPluralMismatchError),
            MessageBody::Plural(plural) => Ok(&plural.msgid_plural),
        }
    }

    pub fn get_msgstr(&self) -> Result<&String, SingularPluralMismatchError> {
        match &self.body {
            MessageBody::Singular(singular) => Ok(&singular.msgstr),
            MessageBody::Plural(_) => Err(SingularPluralMismatchError),
        }
    }

    pub fn get_msgstr_plural(&self) -> Result<&Vec<String>, SingularPluralMismatchError> {
        match &self.body {
            MessageBody::Singular(_) => Err(SingularPluralMismatchError),
            MessageBody::Plural(plural) => Ok(&plural.msgstr_plural),
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
