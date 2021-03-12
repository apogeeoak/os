use super::pretty::Color;
use super::qemu;
use crate::serial_print;
use crate::serial_println;
use core::panic::PanicInfo;

pub trait Testable {
    fn run(&self);
}

impl<T: Fn()> Testable for T {
    fn run(&self) {
        serial_print!("{} ... ", core::any::type_name::<T>());
        self();
        println_ok();
    }
}

// Test runner.
pub fn runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} test(s).", tests.len());
    for &test in tests {
        test.run();
    }
    qemu::exit(qemu::ExitCode::Success)
}

// Test mode panic handler.
pub fn panic(info: &PanicInfo) -> ! {
    println_failed();
    serial_println!("\nError: {}", info);
    qemu::exit(qemu::ExitCode::Failure);
    loop {}
}

fn println_ok() {
    serial_println!("{}", Color::bright_green("ok"));
}

fn println_failed() {
    serial_println!("{}", Color::bright_red("failed"));
}
