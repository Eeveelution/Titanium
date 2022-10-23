#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

#![feature(abi_x86_interrupt)] //enable x86-interrupt

mod titanium;

use core::panic::PanicInfo;

use titanium::qemu::QemuExitCode;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("{}", _info);

    titanium::qemu::exit_qemu(QemuExitCode::Failed);

    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    titanium::interrupts::initialize_idt();

    serial_println!("Testing serial printing!");
    serial_println!("{}", 1.0 / 3.0);

    x86_64::instructions::interrupts::int3();

    titanium::qemu::exit_qemu(QemuExitCode::Success);

    loop {}
}