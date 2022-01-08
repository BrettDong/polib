pub mod parser;
pub mod message;

pub struct SingularMessage {
    pub msgid: String,
    pub msgstr: String,
}

pub struct PluralMessage {
    pub msgid: String,
    pub msgid_plural: String,
    pub msgstr_plural: Vec<String>,
}

pub enum PluralizableMessage {
    Singular(SingularMessage),
    Plural(PluralMessage),
}

pub struct Message {
    pub comments: String,
    pub source: String,
    pub flags: String,
    pub msgctxt: Option<String>,
    pub body: PluralizableMessage,
}

pub struct POFile {
    pub num_plural_forms: usize,
    pub plural_eval: fn(i64) -> usize,
    pub messages: Vec<Message>,
}
