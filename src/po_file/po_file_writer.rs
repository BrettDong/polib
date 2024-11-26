//! Write PO files.

use super::escape::escape;
use crate::catalog::Catalog;
use crate::message::MessageView;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn display_width(content: &str) -> usize {
    content.chars().count()
}

fn wrap(content: &str) -> Vec<&str> {
    let mut spaces: Vec<usize> = content.match_indices(' ').map(|m| m.0 + 1).collect();
    spaces.insert(0, 0);
    if *spaces.last().unwrap() < content.len() {
        spaces.push(content.len());
    }
    let mut spaces = spaces.iter().peekable();
    let mut result: Vec<&str> = Vec::new();
    let mut prev_width = 0;
    let mut prev_index = 0;
    let mut last_line_index = 0;
    while let Some(space) = spaces.next() {
        let begin = *space;
        let end = match spaces.peek() {
            Some(next_space) => **next_space,
            None => {
                break;
            }
        };
        let segment_width = display_width(&content[begin..end]);
        if prev_index == 0 || prev_width + segment_width <= 77 {
            prev_width += segment_width;
            prev_index = end;
        } else {
            result.push(&content[last_line_index..prev_index]);
            last_line_index = prev_index;
            prev_index = end;
            prev_width = segment_width;
        }
    }
    result.push(&content[last_line_index..]);
    result
}

/*
fn wrap(content: &str) -> Vec<String> {
    let mut splits = content.split_inclusive(' ');
    let mut result: Vec<String> = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;
    while let Some(segment) = splits.next() {
        // println!("Segment = \"{}\"", segment);
        let segment_width = display_width(segment);
        // println!("Width = {} -> {}", current_width, current_width + segment_width);
        if current_width + segment_width <= 77 {
            current_width += segment_width;
            current_line.push_str(segment);
        } else {
            result.push(current_line);
            current_line = String::from(segment);
            current_width = segment_width;
        }
    }
    if !current_line.is_empty() {
        result.push(current_line);
    }
    result
}
*/

fn write_field<W: Write>(
    writer: &mut BufWriter<W>,
    field_name: &str,
    content: &str,
) -> Result<(), std::io::Error> {
    let escaped_content = escape(content);
    if content.match_indices('\n').count() <= 1
        && field_name.len() + display_width(escaped_content.as_str()) <= 78
    {
        writer.write_all(field_name.as_bytes())?;
        writer.write_all(b" \"")?;
        writer.write_all(escaped_content.as_bytes())?;
        writer.write_all(b"\"\n")?;
    } else {
        writer.write_all(field_name.as_bytes())?;
        writer.write_all(b" \"\"\n")?;
        let lines: Vec<&str> = escaped_content.split_inclusive("\\n").collect();
        for line in lines {
            let wrapped = wrap(line);
            for folded_line in wrapped {
                writer.write_all(b"\"")?;
                writer.write_all(folded_line.as_bytes())?;
                writer.write_all(b"\"\n")?;
            }
        }
    }
    Ok(())
}

fn write_internal<W: Write>(
    catalog: &Catalog,
    writer: &mut BufWriter<W>,
    comparator: Option<Box<dyn FnMut(&&dyn MessageView, &&dyn MessageView) -> Ordering>>,
) -> Result<(), std::io::Error> {
    writer.write_all(b"\nmsgid \"\"\n")?;
    write_field(writer, "msgstr", catalog.metadata.export_for_po().as_str())?;
    writer.write_all(b"\n")?;

    let messages = if let Some(comparator) = comparator {
        let mut sorting = catalog.messages().collect::<Vec<&dyn MessageView>>();
        sorting.sort_by(comparator);
        sorting
    } else {
        catalog.messages().collect::<Vec<&dyn MessageView>>()
    };

    for message in messages {
        if !message.comments().is_empty() {
            for line in message.comments().split('\n') {
                writer.write_all(b"#. ")?;
                writer.write_all(line.as_bytes())?;
                writer.write_all(b"\n")?;
            }
        }
        if !message.source().is_empty() {
            for line in message.source().split('\n') {
                writer.write_all(b"#: ")?;
                writer.write_all(line.as_bytes())?;
                writer.write_all(b"\n")?;
            }
        }
        if !message.flags().is_empty() {
            writer.write_all(b"#, ")?;
            writer.write_all(message.flags().to_string().as_bytes())?;
            writer.write_all(b"\n")?;
        }
        if let Some(ctxt) = message.msgctxt() {
            write_field(writer, "msgctxt", ctxt)?;
        }
        if message.is_singular() {
            write_field(writer, "msgid", message.msgid())?;
            write_field(writer, "msgstr", message.msgstr().unwrap())?;
        } else {
            write_field(writer, "msgid", message.msgid())?;
            write_field(writer, "msgid_plural", message.msgid_plural().unwrap())?;
            let plurals = message.msgstr_plural().unwrap();
            for (i, plural) in plurals.iter().enumerate() {
                write_field(writer, format!("msgstr[{}]", i).as_str(), plural.as_str())?;
            }
        }
        writer.write_all(b"\n")?;
    }
    writer.flush()?;
    Ok(())
}

/// Writes a catalog in PO format.
pub fn write<W: Write>(catalog: &Catalog, writer: &mut BufWriter<W>) -> Result<(), std::io::Error> {
    write_internal(catalog, writer, None)
}

/// Writes a catalog to a PO file on disk.
pub fn write_to_file(catalog: &Catalog, path: &Path) -> Result<(), std::io::Error> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    write_internal(catalog, &mut writer, None)
}

/// Writes a catalog in PO format with a sorting algorithm.
pub fn write_sort_by<W: Write>(
    catalog: &Catalog,
    writer: &mut BufWriter<W>,
    comparator: Box<dyn FnMut(&&dyn MessageView, &&dyn MessageView) -> Ordering>,
) -> Result<(), std::io::Error> {
    write_internal(catalog, writer, Some(comparator))
}


/// Writes a catalog to a PO file on disk with a sorting algorithm.
pub fn write_to_file_sort_by(
    catalog: &Catalog,
    path: &Path,
    comparator: Box<dyn FnMut(&&dyn MessageView, &&dyn MessageView) -> Ordering>,
) -> Result<(), std::io::Error> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    write_internal(catalog, &mut writer, Some(comparator))
}
