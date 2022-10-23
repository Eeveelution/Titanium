use lazy_static::lazy_static;
use x86_64::{structures::{tss::TaskStateSegment, gdt::{GlobalDescriptorTable, Descriptor, self, SegmentSelector}}, VirtAddr};

pub const DOUBLE_FAULT_STACK_INDEX: u16 = 0;
pub const DOUBLE_FAULT_STACK_SIZE_KB: usize = 64;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        tss.interrupt_stack_table[DOUBLE_FAULT_STACK_INDEX as usize] = {
            const STACK_SIZE: usize = DOUBLE_FAULT_STACK_SIZE_KB * 1024;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;

            stack_end
        };

        return tss;
    };

    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();

        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        
        (gdt, Selectors {code_selector, tss_selector} )
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn initialize() {
    use x86_64::instructions::tables;
    use x86_64::instructions::segmentation::{CS, Segment};

    GDT.0.load();

    unsafe {
        CS::set_reg(GDT.1.code_selector);
        tables::load_tss(GDT.1.tss_selector);
    }
}