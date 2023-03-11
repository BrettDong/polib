use polib::mo_file;
use std::env;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let (input, output) = match (env::args().nth(1), env::args().nth(2)) {
        (Some(input), Some(output)) => (input, output),
        _ => {
            println!("Usage: cargo run --example compile -- <input.po> <output.mo>");
            return Ok(());
        }
    };
    mo_file::compile_from_po(Path::new(&input), Path::new(&output))?;
    Ok(())
}
