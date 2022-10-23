use x86_64::structures::idt::InterruptDescriptorTable;

use lazy_static::lazy_static;

use super::int_breakpoint;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(int_breakpoint);

        return idt;
    };
}

pub fn initialize_idt() {
    IDT.load()
}