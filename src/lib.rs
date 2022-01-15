//! This crate allows users to load, manipulate and save translation data in
//! GNU gettext `.po` file format.
//!
//! This crate is developed for the purpose of using Rust technology stack to
//! do various kinds of data transmission, processing and analysis on
//! translations data in the i18n of another project I am working on. The crate
//! is in early stage of its development and prioritizes the functionalities I
//! need for my data analysis, and might not offer all the functionalities you
//! expect. Feel free to open an issue or even make pull requests if you find
//! some features missing.
//!
//! _Message_ represents an entry in the translation data that maps a string
//! or a pair strings in singular and plural forms in original language to a
//! string or a vector of string in different singular or plural forms in
//! target language.
//!
//! _Catalog_ represents a `.po` file which holds a set of _Messages_.
//!
//! _Metadata_ is the "header" section of a `.po` file that declares its
//! properties like target language, character encoding, translation template
//! creation date, last time translated, plural forms rules, etc.

#![warn(missing_docs)]

pub mod catalog;
pub mod message;
pub mod metadata;
pub mod po_file;
