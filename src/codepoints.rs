use std::io;
use std::str;

/// Wraps a byte-oriented reader and yields the UTF-8 data one code point at a time.
/// Any UTF-8 parsing errors are raised as `io::Error` with `ErrorKind::InvalidData`.
pub struct CodePoints<R: Iterator<Item = io::Result<u8>>> {
    input: R,
    buffer: Vec<u8>,
}

impl<R: Iterator<Item = io::Result<u8>>> From<R> for CodePoints<R> {
    fn from(input: R) -> CodePoints<R> {
        CodePoints {
            input: input,
            buffer: vec![],
        }
    }
}

impl<R: Iterator<Item = io::Result<u8>>> Iterator for CodePoints<R> {
    type Item = io::Result<char>;
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
                        return Some(Err(io::Error::new(io::ErrorKind::InvalidData, "invalid utf8 at end of stream")));
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
