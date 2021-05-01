use spin::Lazy;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;

use super::gdt;
use crate::println;

fn idt() -> &'static InterruptDescriptorTable {
    static INSTANCE: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
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

// Tests
#[cfg(test)]
mod tests {
    #[test_case]
    fn breakpoint_exception() {
        // Ensure a breakpoint exception does not cause a panic.
        x86_64::instructions::interrupts::int3();
    }
}
