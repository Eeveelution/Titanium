use crate::{serial_println, titanium::{self, qemu::QemuExitCode}};

pub fn os_main() {
    titanium::interrupts::interrupt_descriptor_table::initialize();
    titanium::interrupts::global_descriptor_table::initialize();

    serial_println!("Testing serial printing!");
    serial_println!("{}", 1.0 / 3.0);

    serial_println!("\n\nStack overflow test...");

    fn stack_overflow() {
        stack_overflow()
    }

    stack_overflow();

    titanium::qemu::exit_qemu(QemuExitCode::Success);
}