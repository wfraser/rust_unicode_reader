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
