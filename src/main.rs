#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

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
    serial_println!("Testing serial printing!");
    serial_println!("{}", 1.0 / 3.0);

    panic!("AAAAAAAAA");

    titanium::qemu::exit_qemu(QemuExitCode::Success);

    loop {}
}