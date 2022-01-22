#![feature(test)]

extern crate test;
use polib::{
    catalog::Catalog,
    message::Message,
    po_file::{self, po_file_parser::POParseOptions},
};
use std::path::Path;
use test::Bencher;

fn gen_rand_catalog() -> Catalog {
    let lorem = vec!["Lorem ipsum dolor sit amet, consectetur adipiscing elit. ", 
        "Etiam in orci sed quam sollicitudin pretium. ", 
        "Aliquam vulputate tempor tempus. ", 
        "Donec dignissim ante sit amet tellus aliquam venenatis. ", 
        "Aliquam id sollicitudin arcu, placerat mollis turpis. ", 
        "Donec convallis id nulla vel vehicula. ", 
        "Fusce eu gravida eros. ", 
        "Praesent vehicula dui a ultricies tincidunt. ", 
        "Praesent eleifend, ligula vitae egestas aliquet, elit ipsum facilisis velit, vitae gravida nisi metus eget orci. ", 
        "Pellentesque ultricies mauris orci, id lacinia purus tempus in. ", 
        "Etiam a suscipit nulla."
    ];
    let mut catalog = Catalog::new();
    for i in 0..10000 {
        let msgid = match i % 10 {
            0..=3 => lorem[0..3].concat(),
            4..=5 => lorem[0..7].concat(),
            6 => lorem[3..5].concat() + "\"" + lorem[5..6].concat().as_str() + "\"" + lorem[6],
            7 => lorem[0..11].concat(),
            8 => lorem[0..5].concat() + "\n" + lorem[5..10].concat().as_str(),
            9 => lorem[0..5].concat() + "\n" + lorem[5..10].concat().as_str(),
            _ => String::new(),
        };
        if i % 20 == 0 {
            catalog.add_message(Message::new_plural(
                "",
                "foo.c",
                "",
                if i % 100 == 0 { "ctxt" } else { "" },
                format!("{} {}", i, msgid).as_str(),
                format!("{}s {}", i, msgid).as_str(),
                vec![msgid],
            ));
        } else {
            catalog.add_message(Message::new_singular(
                "",
                "foo.c",
                "",
                if i % 100 == 0 { "ctxt" } else { "" },
                format!("{} {}", i, msgid).as_str(),
                msgid.as_str(),
            ));
        }
    }
    catalog
}

#[bench]
fn parse_po_file(bench: &mut Bencher) {
    let path = Path::new("tmp.po");
    let catalog = gen_rand_catalog();
    po_file::write(&catalog, path).unwrap();
    bench.iter(|| {
        let messages = po_file::parse(path, &POParseOptions::default())
            .unwrap()
            .messages;
        messages.len()
    });
    std::fs::remove_file(path).unwrap();
}

#[bench]
fn write_po_file(bench: &mut Bencher) {
    let path = Path::new("tmp.po");
    let catalog = gen_rand_catalog();
    bench.iter(|| {
        po_file::write(&catalog, path).unwrap();
    });
    std::fs::remove_file(path).unwrap();
}
