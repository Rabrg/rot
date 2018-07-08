#![feature(panic_implementation)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// On Linux:
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
