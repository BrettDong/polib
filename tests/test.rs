use polib::po_file;
use std::path::Path;

#[test]
fn parse_sample_po() {
    let path = Path::new("./tests/sample.po");
    let messages = po_file::parse(path).unwrap().messages;
    assert!(messages.len() > 0);
}
