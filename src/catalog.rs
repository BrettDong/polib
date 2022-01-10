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
    pub fn new() -> Self {
        Catalog {
            num_plural_forms: 1,
            plural_eval: |_| 0,
            messages: vec![],
            map: HashMap::new(),
        }
    }

    pub fn find_message_index(&self, msgid: &str) -> Option<&usize> {
        self.map.get(msgid)
    }

    pub fn find_ctxt_message_index(&self, msgctxt: &str, msgid: &str) -> Option<&usize> {
        self.map.get(&gen_internal_key(msgctxt, msgid))
    }

    pub fn update_message_by_index(&mut self, index: usize, message: Message) -> Result<(), &str> {
        if index >= self.messages.len() {
            return Err("Index out of bound!");
        }
        self.map.remove(&self.messages[index].internal_key());
        self.messages[index] = message;
        self.map.insert(self.messages[index].internal_key(), index);
        Ok(())
    }

    pub fn find_message(&self, msgid: &str) -> Option<&Message> {
        match self.find_message_index(msgid) {
            Some(index) => Some(&self.messages[*index]),
            None => None,
        }
    }

    pub fn find_message_ctxt(&self, msgctxt: &str, msgid: &str) -> Option<&Message> {
        match self.find_ctxt_message_index(msgctxt, msgid) {
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

    pub fn translate_ctxt<'a>(&'a self, ctxt: &'a str, query: &'a str) -> &'a str {
        match self.find_message_ctxt(ctxt, query) {
            Some(message) => message.get_msgstr().unwrap(),
            None => query,
        }
    }

    pub fn translate_plural<'a>(
        &'a self,
        query: &'a str,
        query_plural: &'a str,
        n: i64,
    ) -> &'a str {
        match self.find_message(query) {
            Some(message) => &message.get_msgstr_plural().unwrap()[(self.plural_eval)(n)],
            None => {
                if n == 1 {
                    query
                } else {
                    query_plural
                }
            }
        }
    }

    pub fn translate_ctxt_plural<'a>(
        &'a self,
        ctxt: &'a str,
        query: &'a str,
        query_plural: &'a str,
        n: i64,
    ) -> &'a str {
        match self.find_message_ctxt(ctxt, query) {
            Some(message) => &message.get_msgstr_plural().unwrap()[(self.plural_eval)(n)],
            None => {
                if n == 1 {
                    query
                } else {
                    query_plural
                }
            }
        }
    }
}
