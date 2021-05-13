use spin::Lazy;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;

use super::gdt;
use super::keyboard;
use super::pic;
use crate::print;
use crate::println;

fn idt() -> &'static InterruptDescriptorTable {
    static INSTANCE: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::InterruptStackTableIndex::DoubleFault.as_u16());
        }
        idt[pic::InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[pic::InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt
    });

    Lazy::force(&INSTANCE)
}

pub fn init() {
    idt().load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: Breakpoint\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _: u64) -> ! {
    panic!("EXCEPTION: Double Fault\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_: InterruptStackFrame) {
    print!(".");
    pic::end_of_interrupt(pic::InterruptIndex::Timer);
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_: InterruptStackFrame) {
    use keyboard::Key::*;
    match keyboard::read_key() {
        Some(Unicode(character)) => print!("{}", character),
        Some(RawKey(key)) => print!("{:?}", key),
        None => print!("_"),
    }

    pic::end_of_interrupt(pic::InterruptIndex::Keyboard);
}

// Tests
#[cfg(test)]
mod tests {
    #[test_case]
    fn breakpoint_exception() {
        // Ensure a breakpoint exception does not cause a panic.
        x86_64::instructions::interrupts::int3();
    }
}
