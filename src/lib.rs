#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "test_harness"]

pub use library::test;

pub mod library;
pub mod vga_buffer;

mod interrupts;

pub fn init() {
    interrupts::init();
}

#[cfg(test)]
bootloader::entry_point!(tests::main);
#[cfg(test)]
mod tests {
    pub fn main(_: &'static bootloader::BootInfo) -> ! {
        super::init();
        super::test_harness();
        loop {}
    }

    #[panic_handler]
    fn panic(info: &core::panic::PanicInfo) -> ! {
        super::test::panic(info)
    }
}
