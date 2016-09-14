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
//!     let input = "He\u{302}\u{320}llo!";
//!     let mut graphemes = Graphemes::from(Cursor::new(input));
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
