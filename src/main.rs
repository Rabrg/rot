#![feature(panic_implementation)] // required for defining the panic handler
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

#[macro_use]
extern crate lazy_static;
extern crate spin;
extern crate volatile;

use core::panic::PanicInfo;

mod vga_buffer;

/// This function is called on panic.
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;

    writeln!(vga_buffer::WRITER.lock(), "Welcome to Rot OS").unwrap();
    vga_buffer::WRITER.lock().write_string("Hello again");
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    loop {}
}
