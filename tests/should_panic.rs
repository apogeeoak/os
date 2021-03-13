#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test::should_panic::runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

// Entry point.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

// Panic handler.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! { os::test::should_panic::panic(info); }

// Tests.
#[cfg(test)]
mod tests {
    #[test_case]
    pub fn one() {
        let actual = 1;
        assert_eq!(0, actual);
    }

    macro_rules! tests {
        ($($name:ident,)*) => {
            $(
                #[test_case]
                fn $name() {
                    recurse(3_720);
                    let actual = 1;
                    assert_eq!(0, actual);
                }
            )*
        };
    }

    fn recurse(n: u32) {
        let array = [0; 100];
        assert_eq!([0; 100], array);

        match n {
            0 => (),
            n => recurse(n - 1),
        }
    }

    tests! {t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, }
}
