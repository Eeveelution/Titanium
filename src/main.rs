#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod titanium;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut vga_tty = titanium::drivers::vga_tty::create_tty_output();

    titanium::tty::print_string(&mut vga_tty, "Hello, world!\n");
    titanium::tty::print_string(&mut vga_tty, "Titanium TTY test!\n");

    loop {}
}