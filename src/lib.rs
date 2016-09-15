// Copyright (c) 2016 William R. Fraser
// 
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Adaptors which wrap byte-oriented readers and yield the UTF-8 data as Unicode code points or
//! grapheme clusters.
//! 
//! ```rust
//! extern crate unicode_reader;
//! use unicode_reader::Graphemes;
//!
//! use std::io::Cursor;
//!
//! fn main() {
//!     let input = Cursor::new("He\u{302}\u{320}llo!");
//!     let mut graphemes = Graphemes::from(input);
//!     assert_eq!("H", graphemes.next().unwrap().unwrap());
//!     assert_eq!(3,   graphemes.next().unwrap().unwrap().chars().count());
//!     assert_eq!("l", graphemes.next().unwrap().unwrap());
//! }
//! ```
//!
//! [Repository](https://github.com/wfraser/rust_unicode_reader)

#![deny(missing_docs)]

extern crate unicode_segmentation;

mod codepoints;
mod graphemes;

pub use codepoints::CodePoints;
pub use codepoints::BadUtf8Error;
pub use graphemes::Graphemes;

use std::io::{Bytes, Read};

impl<R: Read> From<R> for CodePoints<Bytes<R>> {
    fn from(input: R) -> CodePoints<Bytes<R>> {
        CodePoints::from(input.bytes())
    }
}

impl<R: Read> From<R> for Graphemes<CodePoints<Bytes<R>>> {
    fn from(input: R) -> Graphemes<CodePoints<Bytes<R>>> {
        Graphemes::from(CodePoints::from(input))
    }
}

#[test]
fn test_zalgo() {
    // "zalgo text": The text "ZALGO", with each letter having a ridiculous number of combining
    // marks on it.
    // Should be read in as just 5 ridiculously long graphemes.
    let zalgo =
    "Z\u{0364}\u{0364}\u{033F}\u{034C}\u{0313}\u{0300}\u{0350}\u{0352}\u{030F}\u{0309}\u{0364}\u{0369}\u{0310}\u{0343}\u{0367}\u{034C}\u{0343}\u{0344}\u{035F}\u{032E}\u{0324}\u{032A}\u{033C}\u{032D}\u{031F}\u{0359}\u{032A}\u{0329}\u{0323}\u{0320}\u{032B}\u{0326}\u{0323}\u{0347}\u{0326}\u{0354}\u{0331}A\u{0344}\u{0364}\u{0308}\u{036A}\u{036B}\u{0334}\u{0335}\u{0337}\u{035E}\u{0316}\u{0339}\u{0356}\u{0318}\u{0326}\u{0348}\u{033A}\u{031E}\u{032C}\u{0356}\u{0329}\u{0354}\u{0318}\u{032A}L\u{0312}\u{0342}\u{0357}\u{033E}\u{0343}\u{031A}\u{0301}\u{0346}\u{0334}\u{0328}\u{031C}\u{0329}\u{0349}\u{0318}\u{0349}\u{0359}\u{0329}\u{032A}\u{0355}\u{0359}\u{0332}G\u{0309}\u{0314}\u{030F}\u{036B}\u{030D}\u{036E}\u{030D}\u{0303}\u{036C}\u{030D}\u{0328}\u{031D}\u{0355}\u{035A}\u{0330}\u{0332}\u{032D}O\u{0350}\u{033F}\u{0308}\u{033F}\u{036D}\u{031A}\u{0304}\u{0350}\u{0344}\u{034B}\u{031B}\u{0322}\u{035D}\u{035C}\u{0336}\u{032A}\u{0317}\u{032C}\u{0347}\u{0316}\u{034D}\u{0323}\u{0330}\u{031E}\u{0354}\u{034E}\u{0323}\u{0326}\u{0317}";
    let input = std::io::Cursor::new(zalgo);
    assert_eq!(vec![('Z', 75), ('A', 47), ('L', 43), ('G', 35), ('O' ,59)],
        Graphemes::from(input)
            .map(|g| g.unwrap())
            .map(|g| (g.chars().next().unwrap(), g.len())) // (first_codepoint, num_bytes)
            .collect::<Vec<_>>());
}

#[test]
fn test_utf8_error() {
    let bad = b"a\xe2\x28\xa1bc"; // the 2nd byte of the 3-byte set is invalid utf8
    let mut cps = CodePoints::from(std::io::Cursor::new(bad));
    assert_eq!('a', cps.next().unwrap().unwrap());

    // Here it should fail to parse a code point.
    let result = cps.next().unwrap();
    assert!(result.is_err());
    let ioerr = result.err().unwrap();
    assert_eq!(std::io::ErrorKind::InvalidData, ioerr.kind());
    let inner: Box<BadUtf8Error> = ioerr.into_inner().unwrap().downcast().unwrap();
    assert_eq!(vec![0xe2, 0x28, 0xa1], inner.bytes);

    // It should recover and continue after the bad bytes.
    assert_eq!('b', cps.next().unwrap().unwrap());
    assert_eq!('c', cps.next().unwrap().unwrap());
    assert!(cps.next().is_none());
}
