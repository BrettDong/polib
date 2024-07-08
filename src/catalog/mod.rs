//! Defines `Catalog` struct and its iterators.

mod iterator;

use crate::{
    message::CatalogMessageMutView, message::Message, message::MessageKey,
    metadata::CatalogMetadata,
};
pub use iterator::{Iter, IterMut, MessageMutProxy};
use std::collections::btree_map::BTreeMap;

/// `Catalog` struct represents a collection of _Messages_ stored in a `.po` or `.mo` file.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Catalog {
    /// Metadata of the catalog.
    pub metadata: CatalogMetadata,
    pub(crate) messages: Vec<Option<Message>>,
    pub(crate) map: BTreeMap<MessageKey, usize>,
}

impl Catalog {
    pub(crate) fn empty() -> Self {
        Self {
            metadata: CatalogMetadata::default(),
            messages: vec![],
            map: BTreeMap::new(),
        }
    }

    /// Create a new catalog with a given metadata.
    pub fn new(metadata: CatalogMetadata) -> Self {
        Self {
            metadata,
            ..Self::empty()
        }
    }

    /// Count number of messages in the catalog.
    pub fn count(&self) -> usize {
        self.messages().count()
    }

    /// Is the catalog empty?
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// Get an iterator over immutable messages in the catalog.
    pub fn messages(&self) -> Iter {
        Iter::begin(self)
    }

    /// Get an iterator over messages in the catalog that allows mutating a message in-place.
    pub fn messages_mut(&mut self) -> IterMut {
        IterMut::begin(self)
    }

    /// Find a message in the catalog by msgctxt, msgid and msgid_plural fields. All three fields
    /// have to fully match. Returns None if the message is not found.
    pub fn find_message(
        &self,
        msgctxt: Option<&str>,
        msgid: &str,
        msgid_plural: Option<&str>,
    ) -> Option<&Message> {
        let key = MessageKey::gen(msgctxt, msgid, msgid_plural);
        if let Some(&index) = self.map.get(&key) {
            Some(self.messages[index].as_ref().unwrap())
        } else {
            None
        }
    }

    /// Find a message in the catalog by msgctxt, msgid and msgid_plural fields and get a mutable view.
    /// All three fields have to fully match. Returns None if the message is not found.
    pub fn find_message_mut(
        &mut self,
        msgctxt: Option<&str>,
        msgid: &str,
        msgid_plural: Option<&str>,
    ) -> Option<MessageMutProxy> {
        let key = MessageKey::gen(msgctxt, msgid, msgid_plural);
        if let Some(&index) = self.map.get(&key) {
            Some(MessageMutProxy::at(self, index))
        } else {
            None
        }
    }

    /// Delete a message from the catalog by msgctxt, msgid and msgid_plural fields. All three fields
    /// have to fully match. Returns true if the message is found and deleted. Returns false
    /// if the message does not exist in the catalog.
    pub fn delete_message(
        &mut self,
        msgctxt: Option<&str>,
        msgid: &str,
        msgid_plural: Option<&str>,
    ) -> bool {
        if let Some(mut m) = self.find_message_mut(msgctxt, msgid, msgid_plural) {
            m.delete();
            true
        } else {
            false
        }
    }

    /// Detach a message from the catalog by msgctxt, msgid and msgid_plural fields and get
    /// the `Message` object as return value. If the message does not exist then returns None.
    pub fn detach_message(
        &mut self,
        msgctxt: Option<&str>,
        msgid: &str,
        msgid_plural: Option<&str>,
    ) -> Option<Message> {
        self.find_message_mut(msgctxt, msgid, msgid_plural)
            .map(|mut m| m.detach())
    }

    /// Append a new message to the end of the catalog.
    /// If a message with the exact same `msgctxt`, `msgid` and `msgid_plural` fields already exists
    /// in the catalog, then that message is replaced instead.
    pub fn append_or_update(&mut self, m: Message) {
        let key = MessageKey::from(&m);
        if let Some(&index) = self.map.get(&key) {
            self.messages[index] = Some(m);
        } else {
            let index = self.messages.len();
            self.messages.push(Some(m));
            self.map.insert(key, index);
        }
    }
}
