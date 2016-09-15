// Copyright (c) 2016 William R. Fraser
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;
use std::str;

/// Wraps a byte-oriented reader and yields the UTF-8 data one code point at a time.
/// Any UTF-8 parsing errors are raised as `io::Error` with `ErrorKind::InvalidData`.
pub struct CodePoints<R: Iterator<Item = io::Result<u8>>> {
    input: R,
    buffer: Vec<u8>,
}

impl<R: Iterator<Item = io::Result<u8>>> Iterator for CodePoints<R> {
    /// The type of the elements being iterated over: a `io::Result` with one Unicode code point
    /// (as a `char`), or any I/O error raised by the underlying reader, or any error encountered
    /// while trying to parse the byte stream as UTF-8.
    type Item = io::Result<char>;

    /// Get the next Unicode code point from the stream. Any malformed UTF-8 data will be returned
    /// as an `io::Error` with `ErrorKind::InvalidData`, including if the stream reaches EOF before
    /// a complete code point is read (which is returned as `ErrorKind::UnexpectedEof`). Any I/O
    /// error raised by the underlying stream will be returned as well.
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.input.next() {
                Some(Ok(byte)) => {
                    self.buffer.push(byte);
                },
                None => {
                    if self.buffer.is_empty() {
                        return None;
                    } else {
                        return Some(Err(io::Error::new(io::ErrorKind::UnexpectedEof, "incomplete utf-8 code point at end of stream")));
                    }
                },
                Some(Err(e)) => {
                    return Some(Err(e));
                },
            }
            let maybe_codepoint = match str::from_utf8(&self.buffer) {
                Ok(s) => {
                    let mut chars = s.chars();
                    let c = chars.next().unwrap();
                    assert!(chars.next().is_none(), "unexpectedly got >1 code point at a time!");
                    Some(c)
                },
                Err(e) => {
                    if self.buffer.len() - e.valid_up_to() >= 4 {
                        // If we have 4 bytes that still don't make up a valid code point, then we
                        // have garbage.
                        return Some(Err(io::Error::new(io::ErrorKind::InvalidData, e)));
                    } else {
                        // We probably have a partial code point. Keep reading bytes to find out.
                        None
                    }
                },
            };
            if let Some(codepoint) = maybe_codepoint {
                self.buffer.clear();
                return Some(Ok(codepoint));
            }
        }
    }
}

impl<R: Iterator<Item = io::Result<u8>>> From<R> for CodePoints<R> {
    fn from(input: R) -> CodePoints<R> {
        CodePoints {
            input: input,
            buffer: vec![],
        }
    }
}
