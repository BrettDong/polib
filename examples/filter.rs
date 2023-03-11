use polib::message::{CatalogMessageMutView, MessageView};
use polib::po_file;
use polib::po_file::POParseOptions;
use std::env;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let (input_file, output_file) = match (env::args().nth(1), env::args().nth(2)) {
        (Some(input_file), Some(output_file)) => (input_file, output_file),
        _ => {
            println!("Usage: cargo run --example filter -- <input.po> <output.po>");
            return Ok(());
        }
    };
    let mut catalog = po_file::parse(Path::new(&input_file), &POParseOptions::default())?;
    let mut filtered: usize = 0;
    for mut message in catalog.messages_mut() {
        if !message.is_translated() || message.is_fuzzy() {
            message.delete();
            filtered += 1;
        }
    }
    po_file::write(&catalog, Path::new(&output_file))?;
    println!("{} untranslated or fuzzy translations removed.", filtered);
    Ok(())
}
