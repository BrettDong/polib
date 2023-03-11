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
use polib::po_file::POParseOptions;
use std::error::Error;
use std::path::Path;

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
    Ok(())
}
```

### Remove untranslated or fuzzy entries and save to another `.po` file

```rust
let mut catalog = po_file::parse(Path::new(&input_file), &POParseOptions::default())?;
let mut filtered: usize = 0;
for mut message in catalog.messages_mut() {
    if !message.is_translated() || message.is_fuzzy() {
        message.delete();
        filtered += 1;
    }
}
po_file::write(&catalog, Path::new(&output_file))?;
println!("{} untranslated or fuzzy translations removed.", filtered);
```

### Compile a `.po` file to `.mo` format

```rust
let catalog = po_file::parse(Path::new("in.po"), &POParseOptions::default())?;
mo_file::write(&catalog, Path::new("out.mo"))?;
```

## Documentation

Refer to [docs.rs](https://docs.rs/polib).
