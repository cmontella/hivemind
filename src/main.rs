#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(hivemind::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate hivemind;

use core::panic::PanicInfo;
use hivemind::println;

// This is where it all begins
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    hivemind::init(); // new

    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    hivemind::hlt_loop(); 
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hivemind::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    hivemind::test_panic_handler(info)
}