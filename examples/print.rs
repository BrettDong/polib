use polib::po_file;
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
    let catalog = po_file::parse(Path::new(&path))?;
    for message in catalog.messages {
        if message.is_plural() {
            println!(
                "{} => {}",
                message.get_msgid().unwrap(),
                message.get_msgstr_plural().unwrap()[0]
            );
        } else {
            println!(
                "{} => {}",
                message.get_msgid().unwrap(),
                message.get_msgstr().unwrap()
            );
        }
    }
    Ok(())
}
