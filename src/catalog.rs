//! `Catalog` struct and associated functions.

use crate::{message::*, metadata::CatalogMetadata};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// `Catalog` struct represents translation data stored in a `.po` file.
pub struct Catalog {
    /// Metadata of the catalog.
    pub metadata: CatalogMetadata,
    /// All messages of the catalog.
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

impl Default for Catalog {
    fn default() -> Self {
        Self::new()
    }
}

impl Catalog {
    /// Create a new empty catalog.
    pub fn new() -> Self {
        Catalog {
            metadata: CatalogMetadata::new(),
            messages: vec![],
            map: HashMap::new(),
        }
    }

    /// Total number of messages contained in the catalog, excluding metadata.
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Whether the catalog is empty, i.e. does not contain any message other than metadata.
    pub fn is_empty(&self) -> bool {
        self.len() == 0usize
    }

    /// Find and return the index of a message by msgid,
    /// or None if the message does not exist in the catalog.
    pub fn find_message_index(&self, msgid: &str) -> Option<&usize> {
        self.map.get(msgid)
    }

    /// Find and return the index of a message with context by msgctxt and msgid,
    /// or None if the message does not exist in the catalog.
    pub fn find_ctxt_message_index(&self, msgctxt: &str, msgid: &str) -> Option<&usize> {
        self.map.get(&gen_internal_key(msgctxt, msgid))
    }

    /// Returns the reference to the message at the given index.
    pub fn get_message_by_index(&self, index: usize) -> Option<&Message> {
        self.messages.get(index)
    }

    /// Updates the message at the given index.
    pub fn update_message_by_index(&mut self, index: usize, message: Message) -> Result<(), &str> {
        if index >= self.messages.len() {
            return Err("Index out of bound!");
        }
        self.map.remove(&self.messages[index].internal_key());
        self.messages[index] = message;
        self.map.insert(self.messages[index].internal_key(), index);
        Ok(())
    }

    /// Append a new message to the end of `messages` vector.
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        self.map.insert(
            self.messages.last().unwrap().internal_key(),
            self.messages.len() - 1,
        );
    }

    /// Find and return a reference to the message by msgid,
    /// or None if the message does not exist in the catalog.
    pub fn find_message(&self, msgid: &str) -> Option<&Message> {
        match self.find_message_index(msgid) {
            Some(index) => Some(&self.messages[*index]),
            None => None,
        }
    }

    /// Find and return a reference to the message with context by msgctxt and msgid,
    /// or None if the message does not exist in the catalog.
    pub fn find_ctxt_message(&self, msgctxt: &str, msgid: &str) -> Option<&Message> {
        match self.find_ctxt_message_index(msgctxt, msgid) {
            Some(index) => Some(&self.messages[*index]),
            None => None,
        }
    }
}
