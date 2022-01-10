use crate::catalog::Catalog;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;

pub fn write(catalog: &Catalog, path: &Path) -> Result<(), std::io::Error> {
    let file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .append(false)
        .open(path)?;
    let mut writer = BufWriter::new(file);
    for message in &catalog.messages {
        if !message.comments.is_empty() {
            for line in message.comments.split('\n') {
                writer.write(format!("#. {}\n", line).as_bytes())?;
            }
        }
        if !message.source.is_empty() {
            for line in message.source.split('\n') {
                writer.write(format!("#: {}\n", line).as_bytes())?;
            }
        }
        if !message.flags.is_empty() {
            writer.write(format!("#, {}\n", message.flags).as_bytes())?;
        }
        if !message.msgctxt.is_empty() {
            writer.write(format!("msgctxt \"{}\"\n", message.msgctxt).as_bytes())?;
        }
        if message.is_singular() {
            writer.write(format!("msgid \"{}\"\n", message.get_msgid().unwrap()).as_bytes())?;
            writer.write(format!("msgstr \"{}\"\n", message.get_msgstr().unwrap()).as_bytes())?;
        } else {
            writer.write(format!("msgid \"{}\"\n", message.get_msgid().unwrap()).as_bytes())?;
            writer.write(
                format!("msgid_plural \"{}\"\n", message.get_msgid_plural().unwrap()).as_bytes(),
            )?;
            let plurals = message.get_msgstr_plural().unwrap();
            for i in 0..plurals.len() {
                writer.write(format!("msgstr[{}] \"{}\"\n", i, plurals[i]).as_bytes())?;
            }
        }
        writer.write(b"\n")?;
    }
    Ok(())
}
