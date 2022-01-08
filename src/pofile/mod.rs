use std::collections::HashMap;

pub mod catalog;
pub mod message;
pub mod parser;

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
pub enum PluralizableMessage {
    Singular(SingularMessage),
    Plural(PluralMessage),
}

#[derive(Debug)]
pub struct Message {
    pub comments: String,
    pub source: String,
    pub flags: String,
    pub msgctxt: Option<String>,
    pub body: PluralizableMessage,
}

#[derive(Debug)]
pub struct POFile {
    pub num_plural_forms: usize,
    pub plural_eval: fn(i64) -> usize,
    pub messages: Vec<Message>,
    map: HashMap<String, usize>,
}

#[derive(Debug)]
pub struct SingularPluralMismatchError;
