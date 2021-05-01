use core::cmp;
use core::fmt;
use core::str;

pub struct ByteWriter<const LENGTH: usize> {
    buffer: [u8; LENGTH],
    cursor: usize,
}

impl<const LENGTH: usize> ByteWriter<LENGTH> {
    pub fn new() -> Self {
        let buffer = [0u8; LENGTH];
        ByteWriter { buffer, cursor: 0 }
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        LENGTH
    }

    pub fn clear(&mut self) {
        self.cursor = 0
    }

    pub fn iter(&self) -> core::slice::Iter<u8> {
        self.buffer[..self.cursor].iter()
    }

    pub fn is_empty(&self) -> bool {
        self.cursor == 0
    }

    pub fn is_full(&self) -> bool {
        self.capacity() == self.cursor
    }

    pub fn len(&self) -> usize {
        self.cursor
    }

    pub fn starts_with(&self, bytes: &[u8]) -> bool {
        let len = cmp::min(self.len(), bytes.len());
        self.buffer[..len].iter().eq(bytes.iter())
    }

    pub fn to_str(&self) -> &str {
        str::from_utf8(&self.buffer[..self.cursor]).unwrap()
    }
}

impl<const LENGTH: usize> Default for ByteWriter<LENGTH> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const LENGTH: usize> fmt::Write for ByteWriter<LENGTH> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // Write to buffer ignoring overflow.
        let cap = self.capacity();
        let iter = self.buffer[self.cursor..cap].iter_mut().zip(s.bytes());
        for (i, b) in iter {
            *i = b;
        }
        self.cursor = cmp::min(cap, self.cursor + s.len());
        Ok(())
    }
}
