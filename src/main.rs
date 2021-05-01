#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test::runner)]
#![reexport_test_harness_main = "test_harness"]

#[cfg(not(test))]
bootloader::entry_point!(kernel::main);
#[cfg(not(test))]
mod kernel {
    use os::println;

    pub fn main(_: &'static bootloader::BootInfo) -> ! {
        os::init();
        println!("Hello World!");
        loop {}
    }

    #[panic_handler]
    fn panic(info: &core::panic::PanicInfo) -> ! {
        println!("{}", info);
        loop {}
    }
}

#[cfg(test)]
bootloader::entry_point!(tests::main);
#[cfg(test)]
mod tests {
    pub fn main(_: &'static bootloader::BootInfo) -> ! {
        os::init();
        super::test_harness();
        loop {}
    }

    #[panic_handler]
    fn panic(info: &core::panic::PanicInfo) -> ! {
        os::test::panic(info)
    }
}
