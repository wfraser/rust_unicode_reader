extern crate unicode_reader;
use unicode_reader::*;

use std::io;

fn main() {
    for (i, grapheme) in Graphemes::from(io::stdin()).enumerate() {
        let grapheme = grapheme.expect("I/O error");
        println!("{}: {} ({} code points, {} bytes)",
                i, grapheme, grapheme.chars().count(), grapheme.len());
    }
}
