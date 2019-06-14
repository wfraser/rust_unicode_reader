// Copyright (c) 2016-2019 William R. Fraser
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This crate provides adaptors which wrap byte-oriented readers and yield the UTF-8 data as
//! Unicode code points or grapheme clusters.
//!
//! Unlike other Unicode parsers which work on strings (e.g.
//! [unicode_segmentation](https://crates.io/crates/unicode_segmentation) upon which this is built),
//! this crate works on streams and doesn't require reading the entire data into memory. Instead it
//! yields the graphemes or code points as it reads them.
//!
//! # Example
//!
//! ```rust
//! extern crate unicode_reader;
//! use unicode_reader::{CodePoints, Graphemes};
//!
//! use std::io::Cursor;
//!
//! fn main() {
//!     let input = Cursor::new("He\u{302}\u{320}llo");
//!     let mut graphemes = Graphemes::from(input);
//!     assert_eq!("H",                 graphemes.next().unwrap().unwrap());
//!     assert_eq!("e\u{302}\u{320}",   graphemes.next().unwrap().unwrap()); // note 3 characters
//!     assert_eq!("l",                 graphemes.next().unwrap().unwrap());
//!     assert_eq!("l",                 graphemes.next().unwrap().unwrap());
//!     assert_eq!("o",                 graphemes.next().unwrap().unwrap());
//!     assert!(graphemes.next().is_none());
//!
//!     let greek_bytes = vec![0xCE, 0xA7, 0xCE, 0xB1, 0xCE, 0xAF, 0xCF, 0x81, 0xCE, 0xB5,
//!                            0xCF, 0x84, 0xCE, 0xB5];
//!     let mut codepoints = CodePoints::from(Cursor::new(greek_bytes));
//!     assert_eq!(vec!['Χ', 'α', 'ί', 'ρ', 'ε', 'τ', 'ε'],
//!                 codepoints.map(|r| r.unwrap())
//!                           .collect::<Vec<char>>());
//! }
//! ```
//!
//! [Repository](https://github.com/wfraser/rust_unicode_reader)
//!
//! [Documentation](https://wfraser.github.io/rust_unicode_reader/unicode_reader)

#![deny(missing_docs)]

extern crate unicode_segmentation;

mod codepoints;
mod graphemes;

pub use codepoints::CodePoints;
pub use codepoints::BadUtf8Error;
pub use graphemes::Graphemes;

use std::io::{Bytes, Read};

/// CodePoints can be constructed for any byte-oriented reader.
impl<R: Read> From<R> for CodePoints<Bytes<R>> {
    fn from(input: R) -> CodePoints<Bytes<R>> {
        CodePoints::from(input.bytes())
    }
}

/// Graphemes can be constructed for any byte-oriented reader by going through CodePoints as an
/// internal layer.
impl<R: Read> From<R> for Graphemes<CodePoints<Bytes<R>>> {
    fn from(input: R) -> Graphemes<CodePoints<Bytes<R>>> {
        Graphemes::from(CodePoints::from(input))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::{self, Cursor};

    #[test]
    fn test_zalgo() {
        // "zalgo text": The text "ZALGO", with each letter having a ridiculous number of combining
        // marks on it.
        // Should be read in as just 5 ridiculously long graphemes.
        let zalgo = "Z\u{0364}\u{0364}\u{033F}\u{034C}\u{0313}\u{0300}\u{0350}\u{0352}\u{030F}\u{0309}\u{0364}\u{0369}\u{0310}\u{0343}\u{0367}\u{034C}\u{0343}\u{0344}\u{035F}\u{032E}\u{0324}\u{032A}\u{033C}\u{032D}\u{031F}\u{0359}\u{032A}\u{0329}\u{0323}\u{0320}\u{032B}\u{0326}\u{0323}\u{0347}\u{0326}\u{0354}\u{0331}A\u{0344}\u{0364}\u{0308}\u{036A}\u{036B}\u{0334}\u{0335}\u{0337}\u{035E}\u{0316}\u{0339}\u{0356}\u{0318}\u{0326}\u{0348}\u{033A}\u{031E}\u{032C}\u{0356}\u{0329}\u{0354}\u{0318}\u{032A}L\u{0312}\u{0342}\u{0357}\u{033E}\u{0343}\u{031A}\u{0301}\u{0346}\u{0334}\u{0328}\u{031C}\u{0329}\u{0349}\u{0318}\u{0349}\u{0359}\u{0329}\u{032A}\u{0355}\u{0359}\u{0332}G\u{0309}\u{0314}\u{030F}\u{036B}\u{030D}\u{036E}\u{030D}\u{0303}\u{036C}\u{030D}\u{0328}\u{031D}\u{0355}\u{035A}\u{0330}\u{0332}\u{032D}O\u{0350}\u{033F}\u{0308}\u{033F}\u{036D}\u{031A}\u{0304}\u{0350}\u{0344}\u{034B}\u{031B}\u{0322}\u{035D}\u{035C}\u{0336}\u{032A}\u{0317}\u{032C}\u{0347}\u{0316}\u{034D}\u{0323}\u{0330}\u{031E}\u{0354}\u{034E}\u{0323}\u{0326}\u{0317}";
        let input = Cursor::new(zalgo);
        assert_eq!(vec![('Z', 75), ('A', 47), ('L', 43), ('G', 35), ('O', 59)],
                   Graphemes::from(input)
                .map(|g| g.unwrap())
                .map(|g| (g.chars().next().unwrap(), g.len())) // (first_codepoint, num_bytes)
                .collect::<Vec<_>>());
    }

    fn assert_badutf8err<T>(reader: &mut Iterator<Item = io::Result<T>>,
                            kind: io::ErrorKind,
                            bad_bytes: Vec<u8>) {
        let result = reader.next().unwrap();
        assert!(result.is_err());
        let ioerr: io::Error = result.err().unwrap();
        assert_eq!(kind, ioerr.kind());
        let inner: Box<BadUtf8Error> = ioerr.into_inner().unwrap().downcast().unwrap();
        assert_eq!(bad_bytes, inner.bytes);
    }

    #[test]
    fn test_utf8_error() {
        let bad = b"a\xe2\x28\xa1bc"; // the 2nd byte of the 3-byte set is invalid utf8
        let mut codepoints = CodePoints::from(Cursor::new(bad));
        assert_eq!('a', codepoints.next().unwrap().unwrap());

        // Here it should fail to parse a code point.
        assert_badutf8err(&mut codepoints,
                          io::ErrorKind::InvalidData,
                          vec![0xe2, 0x28, 0xa1]);

        // It should recover and continue after the bad bytes.
        assert_eq!('b', codepoints.next().unwrap().unwrap());
        assert_eq!('c', codepoints.next().unwrap().unwrap());
        assert!(codepoints.next().is_none());
    }

    #[test]
    fn test_error_at_end() {
        let bad = b"a\xe2\x80"; // ends in incomplete 3-byte UTF-8
        let mut codepoints = CodePoints::from(Cursor::new(bad));
        assert_eq!('a', codepoints.next().unwrap().unwrap());

        // Here it should fail to parse a code point.
        assert_badutf8err(&mut codepoints,
                          io::ErrorKind::UnexpectedEof,
                          vec![0xe2, 0x80]);

        // After reading the bad bytes, it should report the end of stream.
        assert!(codepoints.next().is_none());
    }

    #[test]
    fn test_grapheme_bad_utf8() {
        let bad = b"ab\xe2\x28\xa1cd";
        let mut graphemes = Graphemes::from(Cursor::new(bad));

        assert_eq!("a", graphemes.next().unwrap().unwrap());

        // Graphemes will hit the error here, but it needs to store it and return its pending
        // buffer instead.
        assert_eq!("b", graphemes.next().unwrap().unwrap());

        // Now it should raise the error.
        assert_badutf8err(&mut graphemes,
                          io::ErrorKind::InvalidData,
                          vec![0xe2, 0x28, 0xa1]);

        // Now it should recover and return more valid data.
        assert_eq!("c", graphemes.next().unwrap().unwrap());
        assert_eq!("d", graphemes.next().unwrap().unwrap());
        assert!(graphemes.next().is_none());
    }

    #[test]
    fn test_grapheme_bad_utf8_at_start() {
        let bad = b"\xe2\x28\xa1ab";
        let mut graphemes = Graphemes::from(Cursor::new(bad));

        // Now it should raise the error.
        assert_badutf8err(&mut graphemes,
                          io::ErrorKind::InvalidData,
                          vec![0xe2, 0x28, 0xa1]);

        // But recover and read the rest okay.
        assert_eq!("a", graphemes.next().unwrap().unwrap());
        assert_eq!("b", graphemes.next().unwrap().unwrap());
        assert!(graphemes.next().is_none());
    }
}
