//! Iterators over a Catalog.

use crate::catalog::Catalog;
use crate::message::{
    CatalogMessageMutView, Message, MessageFlags, MessageKey, MessageMutView, MessageView,
    SingularPluralMismatchError,
};

pub struct CatalogMessageRef<C> {
    catalog: C,
    index: usize,
}

impl<C> CatalogMessageRef<C> {
    fn begin(catalog: C) -> Self {
        Self { catalog, index: 0 }
    }

    fn at(catalog: C, index: usize) -> Self {
        Self { catalog, index }
    }
}

/// An immutable iterator over messages in a catalog.
pub struct Iter<'a>(CatalogMessageRef<&'a Catalog>);

impl<'a> Iter<'a> {
    pub(crate) fn begin(catalog: &'a Catalog) -> Self {
        Self(CatalogMessageRef::begin(catalog))
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a dyn MessageView;

    fn next(&mut self) -> Option<Self::Item> {
        while self.0.index < self.0.catalog.messages.len() {
            if let Some(m) = self.0.catalog.messages[self.0.index].as_ref() {
                self.0.index += 1;
                return Some(m);
            } else {
                self.0.index += 1
            }
        }
        None
    }
}

/// A mutable iterator over messages in a catalog that allows mutating a message in-place.
pub struct IterMut<'a>(CatalogMessageRef<&'a mut Catalog>);

impl<'a> IterMut<'a> {
    pub(crate) fn begin(catalog: &'a mut Catalog) -> Self {
        Self(CatalogMessageRef::begin(catalog))
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = MessageMutProxy<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.0.index < self.0.catalog.messages.len() {
            if self.0.catalog.messages[self.0.index].is_some() {
                let current_index = self.0.index;
                self.0.index += 1;
                return Some(MessageMutProxy::at(
                    unsafe { &mut *(self.0.catalog as *mut Catalog) },
                    current_index,
                ));
            } else {
                self.0.index += 1;
            }
        }
        None
    }
}

/// Proxy object for mutating a message that belongs to a catalog.
/// Mutating a message in a catalog has to go through this proxy object, otherwise modifying data
/// directly on the `Message` object may cause internal data inconsistencies in the catalog.
pub struct MessageMutProxy<'a>(CatalogMessageRef<&'a mut Catalog>);

impl<'a> MessageMutProxy<'a> {
    pub(crate) fn at(catalog: &'a mut Catalog, index: usize) -> Self {
        Self(CatalogMessageRef::at(catalog, index))
    }

    fn message(&self) -> &Message {
        self.0.catalog.messages[self.0.index].as_ref().unwrap()
    }

    fn message_mut(&mut self) -> &mut Message {
        self.0.catalog.messages[self.0.index].as_mut().unwrap()
    }
}

impl<'a> MessageView for MessageMutProxy<'a> {
    fn is_singular(&self) -> bool {
        self.message().is_singular()
    }

    fn is_plural(&self) -> bool {
        self.message().is_plural()
    }

    fn is_translated(&self) -> bool {
        self.message().is_translated()
    }

    fn is_fuzzy(&self) -> bool {
        self.message().is_fuzzy()
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

    fn msgctxt(&self) -> Option<&str> {
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

impl<'a> MessageMutView for MessageMutProxy<'a> {
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
        self.0.catalog.map.remove(&original_key);
        self.message_mut().msgctxt = msgctxt;
        let new_key = MessageKey::from(self.message());
        self.0.catalog.map.insert(new_key, self.0.index);
    }

    fn set_msgid(&mut self, msgid: String) {
        let original_key = MessageKey::from(self.message());
        self.0.catalog.map.remove(&original_key);
        self.message_mut().msgctxt = msgid;
        let new_key = MessageKey::from(self.message());
        self.0.catalog.map.insert(new_key, self.0.index);
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

impl<'a> CatalogMessageMutView for MessageMutProxy<'a> {
    fn delete(&mut self) {
        let key = MessageKey::from(self.message());
        self.0.catalog.map.remove(&key);
        self.0.catalog.messages[self.0.index] = None;
    }

    fn detach(&mut self) -> Message {
        let key = MessageKey::from(self.message());
        self.0.catalog.map.remove(&key);
        self.0.catalog.messages[self.0.index].take().unwrap()
    }
}
