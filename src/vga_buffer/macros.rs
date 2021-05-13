#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::macros::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;

    // Disable interrupts while writing.
    x86_64::instructions::interrupts::without_interrupts(|| {
        crate::vga_buffer::writer().write_fmt(args).unwrap();
    });
}
