use polib::po_file::{self, po_file_parser::POParseOptions};
use std::path::Path;

#[test]
fn parse_sample_po() {
    let path = Path::new("./tests/sample.po");
    let messages = po_file::parse(path, &POParseOptions::default())
        .unwrap()
        .messages;
    assert!(messages.len() > 0);
}
