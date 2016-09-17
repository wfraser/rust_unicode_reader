# `unicode_reader`

[![Build Status](https://travis-ci.org/wfraser/rust_unicode_reader.svg?branch=master)](https://travis-ci.org/wfraser/rust_unicode_reader)

[Documentation](https://wfraser.github.io/rust_unicode_reader/unicode_reader)

Adaptors which wrap byte-oriented readers and yield the UTF-8 data as Unicode code points or
grapheme clusters.

Unlike other Unicode parsers which work on strings (for instance, 
[unicode_segmentation](https://crates.io/crate/unicode_segmentation), upon which this is built),
this crate works on streams and doesn't require reading the entire data into memory. Instead it
yields the graphemes or code points as it reads them.

```rust
extern crate unicode_reader;
use unicode_reader::Graphemes;

use std::io::Cursor;

fn main() {
    let input = Cursor::new("He\u{302}\u{320}llo");
    let mut graphemes = Graphemes::from(input);
    assert_eq!("H",                 graphemes.next().unwrap().unwrap());
    assert_eq!("e\u{302}\u{320}",   graphemes.next().unwrap().unwrap()); // note 3 characters
    assert_eq!("l",                 graphemes.next().unwrap().unwrap());
    assert_eq!("l",                 graphemes.next().unwrap().unwrap());
    assert_eq!("o",                 graphemes.next().unwrap().unwrap());
    assert!(graphemes.next().is_none());

    let greek_bytes = vec![0xCE, 0xA7, 0xCE, 0xB1, 0xCE, 0xAF, 0xCF, 0x81, 0xCE, 0xB5,
                           0xCF, 0x84, 0xCE, 0xB5];
    let mut codepoints = CodePoints::from(Cursor::new(greek_bytes));
    assert_eq!(vec!['Χ', 'α', 'ί', 'ρ', 'ε', 'τ', 'ε'],
                codepoints.map(|r| r.unwrap())
                          .collect::<Vec<char>>());
}
```

