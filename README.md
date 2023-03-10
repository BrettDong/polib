# `polib`

[![crates.io](https://img.shields.io/crates/v/polib.svg)](https://crates.io/crates/polib)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![GitHub Actions](https://github.com/BrettDong/polib/actions/workflows/test.yaml/badge.svg)

A Rust library to read, manipulate and write GNU gettext translation data in `.po` format.

## Basic Concepts

A **Message** represents a translation from a text entry in source language to a text entry in a target language. 

A **Catalog** holds a collection of _Messages_, and is stored in a `.po` file. 

## Example

### Iterate over messages in a `.po` file

```rust
use polib::po_file;
use polib::po_file::ParseOptions;

fn main() -> Result<(), Box<dyn Error>> {
    let catalog = po_file::parse(Path::new("foo.po"), &POParseOptions::default())?;
    for message in catalog.messages() {
        if message.is_translated() {
            if message.is_singular() {
                println!("{} => {}", message.msgid(), message.msgstr()?);
            } else { // message.is_plural()
                println!("{} => {}", message.msgid(), message.msgstr_plural()?.join(", "));
            }
        } else {
            println!("{} is untranslated", message.msgid());
        }
    }
}
```

## Documentation

Refer to [docs.rs](https://docs.rs/polib).
