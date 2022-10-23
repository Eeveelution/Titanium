use crate::{serial_println, titanium::{self, qemu::QemuExitCode}};

pub fn os_main() {
    titanium::interrupts::initialize_idt();

    serial_println!("Testing serial printing!");
    serial_println!("{}", 1.0 / 3.0);

    x86_64::instructions::interrupts::int3();

    titanium::qemu::exit_qemu(QemuExitCode::Success);
}