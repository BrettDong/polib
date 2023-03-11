//! This crate allows users to load, manipulate and save translation data in
//! GNU gettext `.po` file format. Saving translation data into `.mo` file format
//! is also supported. For simplicity, only UTF-8 encoding is supported.
//!
//! A _Message_ represents an entry in the translation data that maps a string
//! or a pair of singular form and plural form strings in the original language
//! to a string or a vector of string in different plural forms in the target language.
//!
//! A _Catalog_ holds a collection set of _Messages_, and is stored in a `.po` or `.mo` file.
//!
//! _Metadata_ is the "header" section of a _Catalog_ that declares its
//! properties like target language, character encoding, translation template
//! creation date, last time translated, plural forms rules, etc.

#![warn(missing_docs)]

extern crate concat_string;

pub mod catalog;
pub mod message;
pub mod metadata;
pub mod mo_file;
mod plural;
pub mod po_file;
