use x86_64::structures::idt::InterruptStackFrame;

use crate::serial_println;

pub extern "x86-interrupt" fn int_breakpoint(stack_frame: InterruptStackFrame) {
    serial_println!("BREAKPOINT HIT!\n{:#?}", stack_frame)
}