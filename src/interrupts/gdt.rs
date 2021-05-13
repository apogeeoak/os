use spin::Lazy;
use x86_64::structures::gdt::Descriptor;
use x86_64::structures::gdt::GlobalDescriptorTable;
use x86_64::structures::gdt::SegmentSelector;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum InterruptStackTableIndex {
    DoubleFault,
}

struct Gdt {
    gdt: GlobalDescriptorTable,
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

fn gdt() -> &'static Gdt {
    static INSTANCE: Lazy<Gdt> = Lazy::new(|| {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(tss()));
        Gdt { gdt, code_selector, tss_selector }
    });

    Lazy::force(&INSTANCE)
}

fn tss() -> &'static TaskStateSegment {
    static INSTANCE: Lazy<TaskStateSegment> = Lazy::new(|| {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[InterruptStackTableIndex::DoubleFault.as_usize()] = {
            const STACK_SIZE: usize = 4096 * 5;

            #[repr(align(16))]
            struct Stack([u8; STACK_SIZE]);
            static mut STACK: Stack = Stack([0; STACK_SIZE]);

            // Return end of stack (pointer to start + size).
            VirtAddr::from_ptr(unsafe { &STACK }) + STACK_SIZE
        };
        tss
    });

    Lazy::force(&INSTANCE)
}

pub fn init() {
    let gdt = gdt();
    gdt.gdt.load();
    unsafe {
        x86_64::instructions::segmentation::set_cs(gdt.code_selector);
        x86_64::instructions::tables::load_tss(gdt.tss_selector);
    }
}

impl InterruptStackTableIndex {
    pub fn as_u16(self) -> u16 {
        self as u16
    }

    pub fn as_usize(self) -> usize {
        // Prevent lossy conversion.
        usize::from(self.as_u16())
    }
}
