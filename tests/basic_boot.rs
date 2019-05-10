#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(hivemind::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate hivemind;

use core::panic::PanicInfo;
use hivemind::{println, serial_print, serial_println};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    hivemind::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    serial_print!("test_println... ");
    println!("test_println output");
    serial_println!("[ok]");
}