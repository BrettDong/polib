//! Writing to MO files.

mod mo_file_writer;

pub use mo_file_writer::write;

use crate::po_file;
use crate::po_file::POParseOptions;
use std::error::Error;
use std::path::Path;

/// Compile a `.po` file to a `.mo` file.
pub fn compile_from_po(input_path: &Path, output_path: &Path) -> Result<(), Box<dyn Error>> {
    let parse_options = POParseOptions {
        message_body_only: true,
        translated_only: true,
        unsafe_utf8_decode: false,
    };
    let catalog = po_file::parse_with_option(input_path, &parse_options)?;
    write(&catalog, output_path)?;
    Ok(())
}
