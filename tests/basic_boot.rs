#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test::runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os::println;

// Entry point.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

// Panic handler.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test::panic(info)
}

// Tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn println() {
        println!("output");
    }
}
