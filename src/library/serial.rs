use core::fmt;
use spin::{Lazy, Mutex};
use uart_16550::SerialPort;

pub struct Serial {
    inner: &'static Mutex<SerialPort>,
}

pub fn serial() -> Serial {
    static INSTANCE: Lazy<Mutex<SerialPort>> = Lazy::new(|| {
        let mut serial_port = unsafe { SerialPort::new(0x3f8) };
        serial_port.init();
        Mutex::new(serial_port)
    });

    let inner = Lazy::force(&INSTANCE);
    Serial { inner }
}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.inner.lock().write_str(s)
    }
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::library::serial::_print(format_args!($($arg)*)));
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;

    // Disable interrupts while writing.
    x86_64::instructions::interrupts::without_interrupts(|| {
        serial().write_fmt(args).expect("Printing to serial interface failed.");
    })
}
