//! Write MO files.

use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use crate::{catalog::Catalog, message::MessageView};

fn original_repr_len(message: &dyn MessageView) -> usize {
    let mut result = 0usize;
    let ctxt = message.msgctxt();
    if !ctxt.is_empty() {
        result += ctxt.len() + 1;
    }
    result += message.msgid().len();
    if message.is_plural() {
        result += 1 + message.msgid_plural().unwrap().len();
    }
    result
}

fn write_original_repr(
    writer: &mut BufWriter<std::fs::File>,
    message: &dyn MessageView,
) -> Result<(), std::io::Error> {
    let ctxt = message.msgctxt();
    if !ctxt.is_empty() {
        writer.write_all(ctxt.as_bytes())?;
        writer.write_all(&[4u8])?;
    }
    writer.write_all(message.msgid().as_bytes())?;
    if message.is_plural() {
        writer.write_all(&[0u8])?;
        writer.write_all(message.msgid_plural().unwrap().as_bytes())?;
    }
    Ok(())
}

fn translated_repr_len(message: &dyn MessageView) -> usize {
    if let true = message.is_singular() {
        message.msgstr().unwrap().len()
    } else {
        message
            .msgstr_plural()
            .unwrap()
            .iter()
            .map(|x| x.len() + 1)
            .sum::<usize>()
            - 1
    }
}

fn write_translated_repr(
    writer: &mut BufWriter<std::fs::File>,
    message: &dyn MessageView,
) -> Result<(), std::io::Error> {
    if message.is_singular() {
        writer.write_all(message.msgstr().unwrap().as_bytes())?;
    } else {
        writer.write_all(message.msgstr_plural().unwrap().join("\u{0000}").as_bytes())?;
    }
    Ok(())
}

/// Saves a catalog to a binary MO file.
pub fn write(catalog: &Catalog, path: &Path) -> Result<(), std::io::Error> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    let metadata = catalog.metadata.export_for_mo();

    // Header
    let magic_number: u32 = 0x950412de;
    let format_ver: u32 = 0x00000000;
    let num_strings = catalog.count() + 1;
    let orig_table_offset = 28;
    let trans_table_offset = orig_table_offset + 8 * num_strings;
    writer.write_all(&magic_number.to_ne_bytes())?;
    writer.write_all(&format_ver.to_ne_bytes())?;
    writer.write_all(&(num_strings as u32).to_ne_bytes())?;
    writer.write_all(&(orig_table_offset as u32).to_ne_bytes())?;
    writer.write_all(&(trans_table_offset as u32).to_ne_bytes())?;
    writer.write_all(&[0u8; 4])?;
    writer.write_all(&[0u8; 4])?;

    // O table
    let mut offset = trans_table_offset + 8 * num_strings;
    writer.write_all(&0u32.to_ne_bytes())?;
    writer.write_all(&(offset as u32).to_ne_bytes())?;
    offset += 1;
    for &index in catalog.map.values() {
        let length = original_repr_len(catalog.messages[index].as_ref().unwrap());
        writer.write_all(&(length as u32).to_ne_bytes())?;
        writer.write_all(&(offset as u32).to_ne_bytes())?;
        offset += length + 1;
    }

    // T table
    writer.write_all(&(metadata.len() as u32).to_ne_bytes())?;
    writer.write_all(&(offset as u32).to_ne_bytes())?;
    offset += metadata.len() + 1;
    for &index in catalog.map.values() {
        let length = translated_repr_len(catalog.messages[index].as_ref().unwrap());
        writer.write_all(&(length as u32).to_ne_bytes())?;
        writer.write_all(&(offset as u32).to_ne_bytes())?;
        offset += length + 1;
    }

    // O strings
    writer.write_all(&[0u8])?;
    for &index in catalog.map.values() {
        write_original_repr(&mut writer, catalog.messages[index].as_ref().unwrap())?;
        writer.write_all(&[0u8])?;
    }

    // T strings
    writer.write_all(metadata.as_bytes())?;
    writer.write_all(&[0u8])?;
    for &index in catalog.map.values() {
        write_translated_repr(&mut writer, catalog.messages[index].as_ref().unwrap())?;
        writer.write_all(&[0u8])?;
    }

    writer.flush()?;
    Ok(())
}
