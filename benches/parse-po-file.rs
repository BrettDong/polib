#![feature(test)]

extern crate test;

use polib::po_file;
use std::path::Path;
use test::Bencher;

#[bench]
fn parse_po_file(bench: &mut Bencher) {
    let path = Path::new("./tests/sample.po");
    bench.iter(|| {
        let path = Path::new(&path);
        let messages = po_file::parse(path).unwrap().messages;
        messages.len()
    });
}
