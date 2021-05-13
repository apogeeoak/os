use pic8259_simple::ChainedPics;
use spin::Mutex;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = Offset::Pic1 as u8,
    Keyboard,
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum Offset {
    Pic1 = 32,
    Pic2 = Offset::Pic1 as u8 + 8,
}

fn pic() -> &'static Mutex<ChainedPics> {
    static INSTANCE: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(Offset::Pic1 as u8, Offset::Pic2 as u8) });

    &INSTANCE
}

pub fn init() {
    unsafe { pic().lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn end_of_interrupt(interrupt_index: InterruptIndex) {
    unsafe { pic().lock().notify_end_of_interrupt(interrupt_index as u8) };
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_usize(self) -> usize {
        // Prevent lossy conversion.
        usize::from(self.as_u8())
    }
}
