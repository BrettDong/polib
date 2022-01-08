use polib::pofile::*;
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
    let po_file = POFile::parse(Path::new(&path))?;
    for message in po_file.messages {
        match message.body {
            PluralizableMessage::Singular(singular) => {
                println!("{} => {}", &singular.msgid, &singular.msgstr);
            }
            PluralizableMessage::Plural(plural) => {
                println!("{} => {}", &plural.msgid, &plural.msgstr_plural[0]);
            }
        }
    }
    Ok(())
}
