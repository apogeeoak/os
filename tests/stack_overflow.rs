#![no_std]
#![no_main]

// Entry point.
bootloader::entry_point!(main);
fn main(_: &'static bootloader::BootInfo) -> ! {
    os::init();
    os::test::run_should_panic(&stack_overflow);
    loop {}
}

// Panic handler.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let message = b"panicked at 'EXCEPTION: Double Fault";
    os::test::panic_should_panic_with(info, message)
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(&0).read();
}
