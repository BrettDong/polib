//! `Catalog` struct and associated functions.

mod iterator;
use crate::{message::Message, message::MessageKey, metadata::CatalogMetadata};
pub use iterator::{CatalogIterator, CatalogMutableIterator};
use std::collections::btree_map::BTreeMap;

/// `Catalog` struct represents a collection of _Messages_ stored in a `.po` or `.mo` file.
#[derive(Default)]
pub struct Catalog {
    /// Metadata of the catalog.
    pub metadata: CatalogMetadata,
    pub(crate) messages: Vec<Option<Message>>,
    pub(crate) map: BTreeMap<MessageKey, usize>,
}

impl Catalog {
    /// Create a new empty catalog.
    pub fn new() -> Self {
        Self::default()
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
    pub fn messages(&self) -> CatalogIterator {
        CatalogIterator::new(self)
    }

    /// Get an iterator over messages in the catalog that allows mutating a message in-place.
    pub fn messages_mut(&mut self) -> CatalogMutableIterator {
        CatalogMutableIterator::new(self)
    }

    /// Append a new message to the end of the catalog.
    /// If a message with the exact same `msgctxt`, `msgid` and `msgid_plural` field already exists
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
