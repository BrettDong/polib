//! Write MO files.

use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::Path,
};

use crate::{catalog::Catalog, message::Message};

fn original_repr_len(message: &Message) -> usize {
    let mut result = 0usize;
    let ctxt = message.get_msgctxt().unwrap();
    if !ctxt.is_empty() {
        result += ctxt.len() + 1;
    }
    result += message.get_msgid().unwrap().len();
    if message.is_plural() {
        result += 1 + message.get_msgid_plural().unwrap().len();
    }
    result
}

fn gen_original_repr(message: &Message) -> String {
    let mut result = String::new();
    let ctxt = message.get_msgctxt().unwrap();
    if !ctxt.is_empty() {
        result.push_str(ctxt);
        result.push('\u{0004}');
    }
    result.push_str(message.get_msgid().unwrap());
    if message.is_plural() {
        result.push('\u{0000}');
        result.push_str(message.get_msgid_plural().unwrap());
    }
    result
}

fn write_original_repr(
    writer: &mut BufWriter<std::fs::File>,
    message: &Message,
) -> Result<(), std::io::Error> {
    let ctxt = message.get_msgctxt().unwrap();
    if !ctxt.is_empty() {
        writer.write_all(ctxt.as_bytes())?;
        writer.write_all(&[4u8])?;
    }
    writer.write_all(message.get_msgid().unwrap().as_bytes())?;
    if message.is_plural() {
        writer.write_all(&[0u8])?;
        writer.write_all(message.get_msgid_plural().unwrap().as_bytes())?;
    }
    Ok(())
}

fn translated_repr_len(message: &Message) -> usize {
    if let true = message.is_singular() {
        message.get_msgstr().unwrap().len()
    } else {
        message
            .get_msgstr_plural()
            .unwrap()
            .iter()
            .map(|x| x.len() + 1)
            .sum::<usize>()
            - 1
    }
}

fn write_translated_repr(
    writer: &mut BufWriter<std::fs::File>,
    message: &Message,
) -> Result<(), std::io::Error> {
    if message.is_singular() {
        writer.write_all(message.get_msgstr().unwrap().as_bytes())?;
    } else {
        writer.write_all(
            message
                .get_msgstr_plural()
                .unwrap()
                .join("\u{0000}")
                .as_bytes(),
        )?;
    }
    Ok(())
}

/// Saves a catalog to a binary MO file.
pub fn write(catalog: &Catalog, path: &Path) -> Result<(), std::io::Error> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(path)?;

    let mut writer = BufWriter::new(file);

    let mut indices = vec![];

    for i in 0..catalog.messages.len() {
        let message = catalog.get_message_by_index(i).unwrap();
        if message.is_translated() {
            indices.push(i);
        }
    }

    indices.sort_by(|a, b| {
        gen_original_repr(catalog.get_message_by_index(*a).unwrap()).cmp(&gen_original_repr(
            catalog.get_message_by_index(*b).unwrap(),
        ))
    });

    let metadata = catalog.metadata.dump();

    // Header
    let magic_number: u32 = 0x950412de;
    let format_ver: u32 = 0x00000000;
    let num_strings = indices.len() + 1;
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
    for index in &indices {
        let length = original_repr_len(catalog.get_message_by_index(*index).unwrap());
        writer.write_all(&(length as u32).to_ne_bytes())?;
        writer.write_all(&(offset as u32).to_ne_bytes())?;
        offset += length + 1;
    }

    // T table
    writer.write_all(&(metadata.len() as u32).to_ne_bytes())?;
    writer.write_all(&(offset as u32).to_ne_bytes())?;
    offset += metadata.len() + 1;
    for index in &indices {
        let length = translated_repr_len(catalog.get_message_by_index(*index).unwrap());
        writer.write_all(&(length as u32).to_ne_bytes())?;
        writer.write_all(&(offset as u32).to_ne_bytes())?;
        offset += length + 1;
    }

    // O strings
    writer.write_all(&[0u8])?;
    for index in &indices {
        write_original_repr(&mut writer, catalog.get_message_by_index(*index).unwrap())?;
        writer.write_all(&[0u8])?;
    }

    // T strings
    writer.write_all(metadata.as_bytes())?;
    writer.write_all(&[0u8])?;
    for index in &indices {
        write_translated_repr(&mut writer, catalog.get_message_by_index(*index).unwrap())?;
        writer.write_all(&[0u8])?;
    }

    writer.flush()?;
    Ok(())
}
