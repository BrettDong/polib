use std::path::Path;
use polib::pofile::*;

#[test]
fn parse_sample_po() {
    let path = Path::new("./tests/sample.po");
    let messages = POFile::parse(path).unwrap().messages;
    assert!(messages.len() > 0);
}

#[ignore]
#[test]
fn bench() {
    let options = microbench::Options::default();
    let path = Path::new("./tests/sample.po");
    microbench::bench(&options, "parse", || {
        let path = Path::new(&path);
        let messages = POFile::parse(path).unwrap().messages;
        messages.len()
    });
}
