use vga::writers::{Graphics320x240x256, GraphicsWriter};

use crate::{serial_println, titanium::{self}};

pub fn os_main() {
    titanium::interrupts::interrupt_descriptor_table::initialize();
    titanium::interrupts::global_descriptor_table::initialize();

    titanium::drivers::pic8259::initialize();
    titanium::drivers::vga_graphics::initialize();

    x86_64::instructions::interrupts::enable();
    
    serial_println!("Testing serial printing!");
    serial_println!("{}", 1.0 / 3.0);

    let mut i = 0;

    loop {
        i += 1;

        for x in 0..320 {
            for y in 0..240 {
                unsafe {
                    *( (0xA0000 + 320 * y + x) as *mut u8) = i % 255;
                } 
            }
        }

        if i >= 64 {
            i = 0;
        }
    }
    
}