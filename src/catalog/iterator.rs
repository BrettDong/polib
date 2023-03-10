//! Iterators over a Catalog.

use crate::catalog::Catalog;
use crate::message::{
    Message, MessageFlags, MessageKey, MessageView, MutableMessageView, SingularPluralMismatchError,
};

/// An immutable iterator over messages in a catalog.
pub struct CatalogIterator<'a> {
    catalog: &'a Catalog,
    next_index: usize,
}

impl<'a> CatalogIterator<'a> {
    pub(crate) fn new(catalog: &'a Catalog) -> Self {
        Self {
            catalog,
            next_index: 0,
        }
    }
}

impl<'a> Iterator for CatalogIterator<'a> {
    type Item = &'a dyn MessageView;

    fn next(&mut self) -> Option<Self::Item> {
        while self.next_index < self.catalog.messages.len() {
            if let Some(m) = self.catalog.messages[self.next_index].as_ref() {
                self.next_index += 1;
                return Some(m);
            } else {
                self.next_index += 1
            }
        }
        None
    }
}

/// A mutable iterator over messages in a catalog that allows mutating a message in-place.
pub struct CatalogMutableIterator<'a> {
    catalog: &'a mut Catalog,
    current_index: usize,
    next_index: usize,
}

impl<'a> CatalogMutableIterator<'a> {
    pub(crate) fn new(catalog: &'a mut Catalog) -> Self {
        Self {
            catalog,
            current_index: 0,
            next_index: 0,
        }
    }

    fn message(&self) -> &Message {
        self.catalog.messages[self.current_index].as_ref().unwrap()
    }

    fn message_mut(&mut self) -> &mut Message {
        self.catalog.messages[self.current_index].as_mut().unwrap()
    }

    /// Delete this message from the catalog. The message pointed by this proxy object is invalidated.
    /// No further access to the message object pointed by this proxy object is allowed after deletion.
    pub fn delete(&mut self) {
        let key = MessageKey::from(self.message());
        self.catalog.map.remove(&key);
        self.catalog.messages[self.current_index] = None;
    }
}

impl<'a> Iterator for CatalogMutableIterator<'a> {
    type Item = &'a dyn MutableMessageView;

    fn next(&mut self) -> Option<Self::Item> {
        while self.next_index < self.catalog.messages.len() {
            if self.catalog.messages[self.next_index].is_some() {
                self.current_index = self.next_index;
                self.next_index += 1;
                unsafe {
                    let ptr = self as *mut CatalogMutableIterator<'a>;
                    return Some(&*ptr);
                }
            } else {
                self.next_index += 1;
            }
        }
        None
    }
}

impl<'a> MessageView for CatalogMutableIterator<'a> {
    fn is_singular(&self) -> bool {
        self.message().is_singular()
    }

    fn is_plural(&self) -> bool {
        self.message().is_plural()
    }

    fn is_translated(&self) -> bool {
        self.message().is_translated()
    }

    fn comments(&self) -> &str {
        self.message().comments()
    }

    fn source(&self) -> &str {
        self.message().source()
    }

    fn flags(&self) -> &MessageFlags {
        self.message().flags()
    }

    fn msgctxt(&self) -> &str {
        self.message().msgctxt()
    }

    fn msgid(&self) -> &str {
        self.message().msgid()
    }

    fn msgid_plural(&self) -> Result<&str, SingularPluralMismatchError> {
        self.message().msgid_plural()
    }

    fn msgstr(&self) -> Result<&str, SingularPluralMismatchError> {
        self.message().msgstr()
    }

    fn msgstr_plural(&self) -> Result<&Vec<String>, SingularPluralMismatchError> {
        self.message().msgstr_plural()
    }
}

impl<'a> MutableMessageView for CatalogMutableIterator<'a> {
    fn comments_mut(&mut self) -> &mut String {
        &mut self.message_mut().comments
    }

    fn source_mut(&mut self) -> &mut String {
        &mut self.message_mut().source
    }

    fn flags_mut(&mut self) -> &mut MessageFlags {
        &mut self.message_mut().flags
    }

    fn set_msgctxt(&mut self, msgctxt: String) {
        let original_key = MessageKey::from(self.message());
        self.catalog.map.remove(&original_key);
        self.message_mut().msgctxt = msgctxt;
        let new_key = MessageKey::from(self.message());
        self.catalog.map.insert(new_key, self.current_index);
    }

    fn set_msgid(&mut self, msgid: String) {
        let original_key = MessageKey::from(self.message());
        self.catalog.map.remove(&original_key);
        self.message_mut().msgctxt = msgid;
        let new_key = MessageKey::from(self.message());
        self.catalog.map.insert(new_key, self.current_index);
    }

    fn set_msgid_plural(
        &mut self,
        msgid_plural: String,
    ) -> Result<(), SingularPluralMismatchError> {
        if self.message_mut().is_plural() {
            self.message_mut().msgid_plural = msgid_plural;
            Ok(())
        } else {
            Err(SingularPluralMismatchError)
        }
    }

    fn set_msgstr(&mut self, msgstr: String) -> Result<(), SingularPluralMismatchError> {
        if self.message_mut().is_plural() {
            Err(SingularPluralMismatchError)
        } else {
            self.message_mut().msgstr = msgstr;
            Ok(())
        }
    }

    fn msgstr_mut(&mut self) -> Result<&mut String, SingularPluralMismatchError> {
        if self.message_mut().is_plural() {
            Err(SingularPluralMismatchError)
        } else {
            Ok(&mut self.message_mut().msgstr)
        }
    }

    fn msgstr_plural_mut(&mut self) -> Result<&mut Vec<String>, SingularPluralMismatchError> {
        if self.message_mut().is_plural() {
            Ok(&mut self.message_mut().msgstr_plural)
        } else {
            Err(SingularPluralMismatchError)
        }
    }
}
