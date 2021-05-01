#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test::runner)]
#![reexport_test_harness_main = "test_harness"]

use os::println;

// Entry point.
bootloader::entry_point!(main);
fn main(_: &'static bootloader::BootInfo) -> ! {
    os::init();
    test_harness();
    loop {}
}

// Panic handler.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    os::test::panic(info)
}

// Tests.
mod tests {
    use super::*;

    #[test_case]
    fn println() {
        println!("output");
    }
}
