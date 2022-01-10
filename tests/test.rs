use polib::pofile::*;
use std::path::Path;

#[test]
fn parse_sample_po() {
    let path = Path::new("./tests/sample.po");
    let messages = POFile::parse(path).unwrap().messages;
    assert!(messages.len() > 0);
}
