use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::{structures::idt::InterruptStackFrame, instructions::port::Port};

use crate::{serial_print, titanium};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
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

pub fn initialize() {
    unsafe {
        PICS.lock().initialize()
    }
}

//Interrupt handlers
pub extern "x86-interrupt" fn int_pic8259_timer(_stack_frame: InterruptStackFrame) {
    serial_print!(".");

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8())
    }
}

pub extern "x86-interrupt" fn int_pic8259_keyboard(_stack_frame: InterruptStackFrame) {
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    titanium::tty::print(&mut titanium::drivers::vga_tty::create_tty_output(), &[scancode]);

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8())
    }

    serial_print!("{}", scancode);
}
