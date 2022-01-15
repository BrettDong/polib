use crate::{message::*, metadata::CatalogMetadata};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

pub struct Catalog {
    pub metadata: CatalogMetadata,
    pub messages: Vec<Message>,
    pub(crate) map: HashMap<String, usize>,
}

#[derive(Debug)]
pub(crate) struct InvalidCatalogError(pub String);

impl fmt::Display for InvalidCatalogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Catalog error: {}", self.0)
    }
}

impl Error for InvalidCatalogError {}

impl Catalog {
    pub fn new() -> Self {
        Catalog {
            metadata: CatalogMetadata::new(),
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

    pub fn get_message_by_index(&self, index: usize) -> Option<&Message> {
        self.messages.get(index)
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

    pub fn find_ctxt_message(&self, msgctxt: &str, msgid: &str) -> Option<&Message> {
        match self.find_ctxt_message_index(msgctxt, msgid) {
            Some(index) => Some(&self.messages[*index]),
            None => None,
        }
    }
}
