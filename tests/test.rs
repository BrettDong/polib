use polib::po_file;
use std::path::Path;

#[test]
fn parse_sample_po() {
    let path = Path::new("./tests/sample.po");
    let catalog = po_file::parse(path).unwrap();
    assert!(catalog.count() > 0);
}

#[test]
fn parse_sample_po_str() {
    let path = Path::new("./tests/sample.po");
    let text = std::fs::read_to_string(path).unwrap();
    let catalog = po_file::parse_from_reader(text.as_bytes()).unwrap();
    assert!(catalog.count() > 0);
}
