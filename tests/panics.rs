#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Entry point.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    os::test::run_should_panic(&tests::basic);
    loop {}
}

// Panic handler.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test::panic_should_panic(info)
}

// Tests.
#[cfg(test)]
mod tests {
    pub fn basic() {
        let actual = 0;
        assert_eq!(0, actual);
    }
}
