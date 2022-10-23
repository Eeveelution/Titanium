use x86_64::structures::idt::InterruptDescriptorTable;

use lazy_static::lazy_static;

use crate::titanium::drivers::pic8259;

use super::int_breakpoint;
use super::int_double_fault;
use super::global_descriptor_table;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(int_breakpoint);

        unsafe {
            idt.double_fault.set_handler_fn(int_double_fault)
                            .set_stack_index(global_descriptor_table::DOUBLE_FAULT_STACK_INDEX);
        }

        idt[pic8259::InterruptIndex::Timer.as_usize()].set_handler_fn(pic8259::int_pic8259_timer);
        idt[pic8259::InterruptIndex::Keyboard.as_usize()].set_handler_fn(pic8259::int_pic8259_keyboard);

        return idt;
    };
}

pub fn initialize() {
    IDT.load()
}