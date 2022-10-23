use pc_keyboard::{Keyboard, layouts, ScancodeSet1};
use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::{structures::idt::InterruptStackFrame, instructions::port::Port};
use lazy_static::lazy_static;

use crate::{serial_print, serial_println};

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
    //serial_print!(".");

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8())
    }
}

pub extern "x86-interrupt" fn int_pic8259_keyboard(_stack_frame: InterruptStackFrame) {
    use pc_keyboard::{DecodedKey, HandleControl};
    
    lazy_static! {
        static ref KEYBOARD: Mutex< Keyboard<layouts::Us104Key, ScancodeSet1> > = 
            Mutex::new( Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore) );
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe {
        port.read()
    };

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => serial_print!("{}", character),
                DecodedKey::RawKey(key) => serial_print!("{:?}", key),
            }
        }
    }
}
