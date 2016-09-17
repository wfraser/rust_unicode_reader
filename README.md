# `unicode_reader`

[![Build Status](https://travis-ci.org/wfraser/rust_unicode_reader.svg?branch=master)](https://travis-ci.org/wfraser/rust_unicode_reader)

[Documentation](https://wfraser.github.io/rust_unicode_reader/unicode_reader)

Adaptors which wrap byte-oriented readers and yield the UTF-8 data as Unicode code points or
grapheme clusters.

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
}
```

