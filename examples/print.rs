use polib::po_file;
use polib::po_file::POParseOptions;
use std::env;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let path = match env::args().nth(1) {
        Some(str) => str,
        None => {
            println!("Usage: cargo run --example print -- <path to .po>");
            return Ok(());
        }
    };
    let mut catalog = po_file::parse(Path::new(&path), &POParseOptions::default())?;
    for message in catalog.messages_mut() {
        if message.is_translated() {
            println!(
                "{} => {}",
                message.msgid(),
                match message.is_plural() {
                    true => message.msgstr_plural()?.join(", "),
                    false => message.msgstr()?.to_string(),
                }
            );
        }
    }
    Ok(())
}
