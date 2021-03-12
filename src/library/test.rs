use super::pretty::Color;
use super::qemu;
use crate::serial_print;
use crate::serial_println;
use core::panic::PanicInfo;

pub trait Testable {
    fn run(&self);
    fn run_should_panic(&self);
}

impl<T: Fn()> Testable for T {
    fn run(&self) {
        serial_print!("{} ... ", core::any::type_name::<T>());
        self();
        println_ok();
    }

    fn run_should_panic(&self) {
        serial_print!("{} ... ", core::any::type_name::<T>());
        self();
        println_failed();
        qemu::exit(qemu::ExitCode::Failure);
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

// Run single should panic test.
pub fn run_should_panic(test: &dyn Testable) {
    serial_println!("Running should panic test.");
    test.run_should_panic();
}

// Test mode panic handler.
pub fn panic(info: &PanicInfo) -> ! {
    println_failed();
    serial_println!("\nError: {}", info);
    qemu::exit(qemu::ExitCode::Failure);
    loop {}
}

// Test mode should panic handler.
pub fn panic_should_panic(_: &PanicInfo) -> ! {
    println_ok();
    qemu::exit(qemu::ExitCode::Success);
    loop {}
}

fn println_ok() {
    serial_println!("{}", Color::bright_green("ok"));
}

fn println_failed() {
    serial_println!("{}", Color::bright_red("failed"));
}
