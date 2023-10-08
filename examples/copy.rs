use polib::po_file;
use std::env;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let (input, output) = match (env::args().nth(1), env::args().nth(2)) {
        (Some(input), Some(output)) => (input, output),
        _ => {
            println!("Usage: cargo run --example copy -- <input.po> <output.po>");
            return Ok(());
        }
    };
    let catalog = po_file::parse(Path::new(&input))?;
    po_file::write_to_file(&catalog, Path::new(&output))?;
    Ok(())
}
