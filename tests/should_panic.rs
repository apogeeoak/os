#![no_std]
#![no_main]

// Entry point.
bootloader::entry_point!(main);
fn main(_: &'static bootloader::BootInfo) -> ! {
    os::init();
    os::test::run_should_panic(&tests::basic);
    loop {}
}

// Panic handler.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    os::test::panic_should_panic(info)
}

// Tests.
mod tests {
    pub fn basic() {
        let actual = 1;
        assert_eq!(0, actual);
    }
}
