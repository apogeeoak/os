#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub mod vga_buffer;

// Entry point.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");
    #[allow(clippy::empty_loop)]
    loop {}
}

// Panic handler.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
