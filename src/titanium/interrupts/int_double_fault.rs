use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn int_double_fault(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    panic!("MAJOR EXCEPTION! DOUBLE FAULT OCCURED...\nERROR CODE: {}\n{:#?}", error_code, stack_frame)
}