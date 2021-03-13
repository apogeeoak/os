use super::qemu;
use crate::serial_print;
use crate::serial_println;
use core::panic::PanicInfo;
use core::slice::Iter;
use spin::Mutex;
use spin::Once;

/// Implementation Incomplete: Stack is not properly aligned after
/// running tests.

pub trait Testable: Sync {
    fn run(&self);
}

impl<T: Fn() + Sync> Testable for T {
    fn run(&self) {
        serial_print!("{} ... ", core::any::type_name::<T>());
        self();
        serial_println!("failed");
        qemu::exit(qemu::ExitCode::Failure);
    }
}

// Test runner.
pub fn runner(tests: &'static [&dyn Testable]) {
    serial_println!("Running {} test(s).", tests.len());

    // Save tests and stack pointer.
    let rsp: u64;
    unsafe {
        asm!("mov {}, rsp", out(reg) rsp);
    }
    Tests::init(tests, rsp);

    run_next();
}

// Test mode panic handler.
pub fn panic(_: &PanicInfo) -> ! {
    serial_println!("ok");

    // Reset stack pointer.
    if let Some(tests) = Tests::get() {
        let rsp = tests.stack_pointer();
        unsafe { asm!("mov rsp, {}", in(reg) rsp) }
    }

    run_next();
    loop {}
}

fn run_next() {
    let next = Tests::get().and_then(|mut i| i.next());
    match next {
        // All test ran successfully.
        None => qemu::exit(qemu::ExitCode::Success),
        // Run next test.
        Some(&test) => test.run(),
    }
}

struct TestsIter {
    iter: Iter<'static, &'static dyn Testable>,
    stack_pointer: u64,
}

impl TestsIter {
    fn new(items: &'static [&dyn Testable], stack_pointer: u64) -> TestsIter {
        let iter = items.iter();
        TestsIter { iter, stack_pointer }
    }
}

struct Tests {
    inner: &'static Mutex<TestsIter>,
}

impl Iterator for Tests {
    type Item = &'static &'static dyn Testable;

    fn next(&mut self) -> Option<Self::Item> { self.inner.lock().iter.next() }
}

impl Tests {
    fn instance() -> &'static Once<Mutex<TestsIter>> {
        static INSTANCE: Once<Mutex<TestsIter>> = Once::new();
        &INSTANCE
    }

    pub fn init(items: &'static [&dyn Testable], stack_pointer: u64) -> Tests {
        let inner =
            Tests::instance().call_once(|| Mutex::new(TestsIter::new(items, stack_pointer)));
        Tests { inner }
    }

    pub fn get() -> Option<Tests> { Tests::instance().get().map(|inner| Tests { inner }) }

    pub fn stack_pointer(&self) -> u64 { self.inner.lock().stack_pointer }
}
