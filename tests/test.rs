use polib::catalog::Catalog;
use polib::po_file;
use std::path::Path;

fn validate_catalog(catalog: &Catalog) {
    assert_eq!(catalog.count(), 4);
    let mut index: usize = 0;
    for message in catalog.messages() {
        if index == 0 {
            assert!(message.msgctxt().is_none());
            assert_eq!(message.msgid(), "Hello");
            assert!(message.is_singular());
            assert!(message.is_translated());
            assert_eq!(message.msgstr().unwrap_or("None?"), "Translated_Hello");
        } else if index == 1 {
            assert!(message.msgctxt().is_none());
            assert_eq!(message.msgid(), "Book");
            assert!(message.is_plural());
            assert!(message.is_translated());
            assert_eq!(message.msgid_plural().unwrap_or("None?"), "Books");
            assert_eq!(
                message
                    .msgstr_plural()
                    .unwrap_or(&vec![String::from("None?")])[0],
                "Translated\n_\nBook"
            );
        } else if index == 2 {
            assert_eq!(message.msgctxt().unwrap_or("None?"), "Context");
            assert_eq!(message.msgid(), "Good");
            assert!(message.is_singular());
            assert!(message.is_translated());
            assert_eq!(message.msgstr().unwrap_or("None?"), "\"\n\"123");
        } else if index == 3 {
            assert!(message.msgctxt().is_none());
            assert_eq!(message.msgid(), "Untranslated");
            assert!(!message.is_translated());
            assert_eq!(message.msgstr().unwrap_or("None?"), "");
        }

        index += 1;
    }
}

fn feed_test_po() -> Vec<u8> {
    let path = Path::new("./tests/sample.po");
    let text = std::fs::read_to_string(path).unwrap();
    return text.as_bytes().into();
}

#[test]
fn parse_sample_po() {
    let catalog = po_file::parse_from_reader(&*feed_test_po()).unwrap();
    validate_catalog(&catalog);
}

#[test]
fn po_round_trip() {
    let catalog = po_file::parse_from_reader(&*feed_test_po()).unwrap();
    let mut writer = std::io::BufWriter::new(Vec::new());
    po_file::write(&catalog, &mut writer).unwrap();
    let po_bytes = writer.into_inner().unwrap();
    let catalog_2 = po_file::parse_from_reader(&*po_bytes).unwrap();
    validate_catalog(&catalog_2);
}

#[test]
fn po_round_trip_sort() {
    let catalog = po_file::parse_from_reader(&*feed_test_po()).unwrap();
    let mut writer = std::io::BufWriter::new(Vec::new());
    po_file::write_sort_by(&catalog, &mut writer, Box::new(|a, b| {
        a.source().cmp(b.source())
    })).unwrap();
    let po_bytes = writer.into_inner().unwrap();
    let catalog_2 = po_file::parse_from_reader(&*po_bytes).unwrap();
    validate_catalog(&catalog_2);
}
