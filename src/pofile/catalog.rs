use crate::pofile::*;

use super::message::gen_internal_key;

impl POFile {
    pub fn find_message(&self, msgid: &str) -> Option<&Message> {
        match self.map.get(msgid) {
            Some(index) => Some(&self.messages[*index]),
            None => None,
        }
    }

    pub fn find_message_ctxt(&self, msgctxt: &str, msgid: &str) -> Option<&Message> {
        match self.map.get(&gen_internal_key(&msgctxt, &msgid)) {
            Some(index) => Some(&self.messages[*index]),
            None => None,
        }
    }

    pub fn translate<'a>(&'a self, query: &'a str) -> &'a str {
        match self.find_message(&query) {
            Some(message) => message.get_msgstr().unwrap(),
            None => query,
        }
    }
}
