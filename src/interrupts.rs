mod gdt;
mod idt;
mod keyboard;
mod pic;

pub fn init() {
    gdt::init();
    idt::init();
    pic::init();
}
