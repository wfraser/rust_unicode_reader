// Copyright (c) 2016-2019 William R. Fraser
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error::Error;
use std::fmt;
use std::io;
use std::mem;
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
            if !self.buffer.is_empty() {
                // See if we have a valid codepoint.
                let maybe_codepoint = match str::from_utf8(&self.buffer) {
                    Ok(s) => {
                        let mut chars = s.chars();
                        let c = chars.next().unwrap();
                        assert!(chars.next().is_none(),
                                "unexpectedly got >1 code point at a time!");
                        Ok(Some(c))
                    }
                    Err(e) => {
                        if self.buffer.len() - e.valid_up_to() >= 4 {
                            // If we have 4 bytes that still don't make up a valid code point, then
                            // we have garbage.
                            Err(())
                        } else {
                            // We probably have a partial code point. Keep reading bytes to find
                            // out.
                            Ok(None)
                        }
                    }
                };
                match maybe_codepoint {
                    Ok(Some(codepoint)) => {
                        self.buffer.clear();
                        return Some(Ok(codepoint));
                    }
                    Err(()) => {
                        // We have bad data in the buffer. Remove leading bytes until either the
                        // buffer is empty, or we have a valid code point.
                        let mut badbytes: Vec<u8> = vec![];
                        loop {
                            self.buffer = {
                                let (first, rest) = self.buffer.split_first().unwrap();
                                badbytes.push(*first);
                                rest.to_owned()
                            };
                            if self.buffer.is_empty() || str::from_utf8(&self.buffer).is_ok() {
                                break;
                            }
                        }
                        // Raise the error. If we still have data in the buffer, it will be
                        // returned on the next loop.
                        return Some(Err(io::Error::new(io::ErrorKind::InvalidData,
                                                       BadUtf8Error { bytes: badbytes })));
                    }
                    Ok(None) => (),
                }
            }
            match self.input.next() {
                Some(Ok(byte)) => {
                    self.buffer.push(byte);
                }
                None => {
                    if self.buffer.is_empty() {
                        return None;
                    } else {
                        // Invalid utf-8 at end of stream.
                        return Some(Err(io::Error::new(io::ErrorKind::UnexpectedEof,
                                                       BadUtf8Error {
                                                           bytes: mem::replace(&mut self.buffer,
                                                                               vec![]),
                                                       })));
                    }
                }
                Some(Err(e)) => {
                    return Some(Err(e));
                }
            }
        }
    }
}

impl<R: Iterator<Item = io::Result<u8>>> From<R> for CodePoints<R> {
    fn from(input: R) -> CodePoints<R> {
        CodePoints {
            input,
            buffer: vec![],
        }
    }
}

/// An error raised when parsing a UTF-8 byte stream fails.
#[derive(Debug)]
pub struct BadUtf8Error {
    /// The bytes that could not be parsed as a code point.
    pub bytes: Vec<u8>,
}

impl Error for BadUtf8Error {
    fn description(&self) -> &str {
        "BadUtf8Error"
    }
}

impl fmt::Display for BadUtf8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bad UTF-8: {:?}", self.bytes)
    }
}
