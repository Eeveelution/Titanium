#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod titanium;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVW";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    titanium::tty::print(HELLO);

    loop {}
}