use unicode_segmentation::UnicodeSegmentation;
use std::io;
use std::mem;

/// Wraps a `char`-oriented reader and yields the data one Unicode grapheme cluster at a time.
pub struct Graphemes<R: Iterator<Item = io::Result<char>>> {
    input: R,
    buffer: String,
}

impl<R: Iterator<Item = io::Result<char>>> From<R> for Graphemes<R> {
    fn from(input: R) -> Graphemes<R> {
        Graphemes {
            input: input,
            buffer: String::new(),
        }
    }
}

impl<R: Iterator<Item = io::Result<char>>> Iterator for Graphemes<R> {
    type Item = io::Result<String>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.input.next() {
                Some(Ok(codepoint)) => {
                    self.buffer.push(codepoint);
                },
                None => {
                    if self.buffer.is_empty() {
                        return None;
                    } else {
                        return Some(Ok(mem::replace(&mut self.buffer, String::new())));
                    }
                },
                Some(Err(e)) => {
                    return Some(Err(e));
                },
            }
            let grapheme_length_pair = {
                let mut gi = self.buffer.grapheme_indices(true).fuse();
                let first = gi.next();
                let second = gi.next();
                if first.is_some() && second.is_some() {
                    let grapheme = first.unwrap().1.to_owned();
                    let len = second.unwrap().0;
                    Some((grapheme, len))
                } else {
                    // Until we have two graphemes, we can't be sure there won't be combining marks
                    // ahead. Keep reading.
                    None
                }
            };
            if let Some((grapheme, length)) = grapheme_length_pair {
                self.buffer = unsafe { self.buffer.slice_unchecked(length, self.buffer.len()) }.to_owned();
                return Some(Ok(grapheme));
            }
        }
    }
}
