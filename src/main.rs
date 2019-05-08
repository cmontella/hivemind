#![no_std]
#![no_main]

extern crate lazy_static;
extern crate spin;
extern crate volatile;

use core::panic::PanicInfo;
use core::fmt::Write;

mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
