use core::fmt;
use spin::{Lazy, Mutex, MutexGuard};
use super::buffer::BufferWriter;

pub struct Writer {
    inner: &'static Mutex<BufferWriter>,
}

pub struct WriterLock<'a> {
    inner: MutexGuard<'a, BufferWriter>,
}

pub fn writer() -> Writer {
    static INSTANCE: Lazy<Mutex<BufferWriter>> = Lazy::new(|| Mutex::new(BufferWriter::new()));

    let inner = Lazy::force(&INSTANCE);
    Writer { inner }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

impl Writer {
    pub fn lock(&self) -> WriterLock {
        WriterLock {
            inner: self.inner.lock(),
        }
    }

    pub fn read_byte(&self, row: usize, column: usize) -> u8 {
        self.lock().read_byte(row, column)
    }

    pub fn write_byte(&self, byte: u8) {
        self.lock().write_byte(byte);
    }

    pub fn write_string(&self, s: &str) {
        self.lock().write_string(s);
    }
}

impl WriterLock<'_> {
    pub fn read_byte(&self, row: usize, column: usize) -> u8 {
        self.inner.read_byte(row, column)
    }

    fn write_byte(&mut self, byte: u8) {
        self.inner.write_byte(byte);
    }

    fn write_string(&mut self, s: &str) {
        self.inner.write_string(s);
    }
}
