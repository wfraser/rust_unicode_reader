// Copyright (c) 2016-2019 William R. Fraser
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use unicode_segmentation::UnicodeSegmentation;
use std::io;
use std::mem;

/// Wraps a `char`-oriented reader and yields the data one Unicode grapheme cluster at a time.
pub struct Graphemes<R: Iterator<Item = io::Result<char>>> {
    input: R,
    buffer: String,
    pending_error: Option<io::Error>,
}

impl<R: Iterator<Item = io::Result<char>>> Iterator for Graphemes<R> {
    /// The type of the elements being iterated over: a `io::Result` with one Unicode grapheme
    /// cluster, or any I/O error raised by the underlying reader.
    type Item = io::Result<String>;

    /// Get the next grapheme cluster from the stream. Note that because grapheme clusters are of
    /// indeterminate length, this has to read the underlying reader until the *next* cluster
    /// starts before it can return a grapheme.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(err) = self.pending_error.take() {
            return Some(Err(err));
        }
        loop {
            match self.input.next() {
                Some(Ok(codepoint)) => {
                    self.buffer.push(codepoint);
                }
                None => {
                    if self.buffer.is_empty() {
                        return None;
                    } else {
                        return Some(Ok(mem::replace(&mut self.buffer, String::new())));
                    }
                }
                Some(Err(e)) => {
                    if self.buffer.is_empty() {
                        return Some(Err(e));
                    } else {
                        // If the buffer is non-empty, consider the grapheme done and return it,
                        // but save the error and raise it next time around.
                        self.pending_error = Some(e);
                        return Some(Ok(mem::replace(&mut self.buffer, String::new())));
                    }
                }
            }

            let mut gi = self.buffer.grapheme_indices(true).fuse();
            if let (Some((_, first_grapheme)), Some((second_pos, _))) = (gi.next(), gi.next()) {
                let grapheme = first_grapheme.to_owned();
                self.buffer = unsafe { self.buffer.get_unchecked(second_pos ..) }.to_owned();
                return Some(Ok(grapheme));
            }
            // Otherwise, keep reading. We need at least the start of a second grapheme in the
            // buffer before we know where the first one ends, because otherwise there could be
            // additional combining marks ahead.
        }
    }
}

impl<R: Iterator<Item = io::Result<char>>> From<R> for Graphemes<R> {
    fn from(input: R) -> Graphemes<R> {
        Graphemes {
            input,
            buffer: String::new(),
            pending_error: None,
        }
    }
}
