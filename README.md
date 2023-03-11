# `polib`

[![crates.io](https://img.shields.io/crates/v/polib.svg)](https://crates.io/crates/polib)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![GitHub Actions](https://github.com/BrettDong/polib/actions/workflows/test.yaml/badge.svg)

A Rust library to read, manipulate and write GNU gettext translation data in `.po` format.

## Basic Concepts

A **Message** represents a translation of a text entry from the source language to a target language. 

A **Catalog** holds a collection of _Messages_, and is stored in a `.po` file. 

## Example

### Iterate over messages in a `.po` file

```rust
use polib::po_file;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let catalog = po_file::parse(Path::new("foo.po"))?;
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
let mut catalog = po_file::parse(Path::new(&input_file))?;
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

### Fill in missing translations from some other translation service

```rust
let mut catalog = po_file::parse(Path::new(&input_file))?;
for mut message in catalog.messages_mut() {
    if !message.is_translated() {
        if message.is_singular() {
            message.set_msgstr(/* some 3rdparty provided */translate(message.msgid()))?;
        }
    }
}
po_file::write(&catalog, Path::new(&output_file))?;
```

### Compile a `.po` file to `.mo` format

```rust
mo_file::compile_from_po(Path::new(&input), Path::new(&output))?;
```

## Documentation

Refer to [docs.rs](https://docs.rs/polib).
