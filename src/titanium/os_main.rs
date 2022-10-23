use crate::{serial_println, titanium::{self}};

pub fn os_main() {
    titanium::interrupts::interrupt_descriptor_table::initialize();
    titanium::interrupts::global_descriptor_table::initialize();
    titanium::drivers::pic8259::initialize();

    x86_64::instructions::interrupts::enable();
    
    serial_println!("Testing serial printing!");
    serial_println!("{}", 1.0 / 3.0);
}