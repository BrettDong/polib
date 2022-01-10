use crate::message::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Catalog {
    pub num_plural_forms: usize,
    pub plural_eval: fn(i64) -> usize,
    pub messages: Vec<Message>,
    pub(crate) map: HashMap<String, usize>,
}

impl Catalog {
    pub fn find_message(&self, msgid: &str) -> Option<&Message> {
        match self.map.get(msgid) {
            Some(index) => Some(&self.messages[*index]),
            None => None,
        }
    }

    pub fn find_message_ctxt(&self, msgctxt: &str, msgid: &str) -> Option<&Message> {
        match self.map.get(&gen_internal_key(msgctxt, msgid)) {
            Some(index) => Some(&self.messages[*index]),
            None => None,
        }
    }

    pub fn translate<'a>(&'a self, query: &'a str) -> &'a str {
        match self.find_message(query) {
            Some(message) => message.get_msgstr().unwrap(),
            None => query,
        }
    }
}
