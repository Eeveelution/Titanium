#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

#![feature(abi_x86_interrupt)] //enable x86-interrupt
#![feature(custom_test_frameworks)] //enables selfmade test frameworks

#![test_runner(crate::titanium::testing::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod titanium;
use titanium::qemu::QemuExitCode;
use x86_64::instructions;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("{}", _info);

    titanium::qemu::exit_qemu(QemuExitCode::Failed);

    halt_loop()
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    titanium::os_main();

    halt_loop()
}

pub fn halt_loop() -> ! {
    loop {
        instructions::hlt();
    }
}