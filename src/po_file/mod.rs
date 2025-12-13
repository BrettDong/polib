//! Parsing and writing to PO files.

mod escape;
mod po_file_parser;
mod po_file_writer;

pub use po_file_parser::{
    parse, parse_from_reader, parse_from_reader_with_option, parse_with_option,
};
pub use po_file_parser::{POParseError, POParseOptions};
pub use po_file_writer::{write, write_to_file, write_sort_by, write_to_file_sort_by};
