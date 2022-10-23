use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::structures::idt::InterruptStackFrame;

use crate::serial_println;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub static PICS: Mutex< ChainedPics > = Mutex::new( unsafe {
    ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
});

pub extern "x86-interrupt" fn int_pic8259(_stack_frame: InterruptStackFrame) {
    serial_println!("Beyley is cute!");

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8())
    }
}

pub fn initialize() {
    unsafe {
        PICS.lock().initialize()
    }
}