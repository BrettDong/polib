use polib::po_file;
use polib::po_file::POParseOptions;
use std::path::Path;

#[test]
fn parse_sample_po() {
    let path = Path::new("./tests/sample.po");
    let catalog = po_file::parse(path, &POParseOptions::default()).unwrap();
    assert!(catalog.count() > 0);
}
