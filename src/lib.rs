//! This crate allows users to load, manipulate and save translation data in
//! GNU gettext `.po` file format.
//!
//! _Message_ represents an entry in the translation data that maps a string
//! or a pair strings in singular and plural forms in original language to a
//! string or a vector of string in different singular or plural forms in
//! target language.
//!
//! _Catalog_ represents the set of _Messages_ stored in a `.po` or `.mo` file.
//!
//! _Metadata_ is the "header" section of a _Catalog_ that declares its
//! properties like target language, character encoding, translation template
//! creation date, last time translated, plural forms rules, etc.

#![warn(missing_docs)]

pub mod catalog;
pub mod message;
pub mod metadata;
pub mod mo_file;
mod plural;
pub mod po_file;
