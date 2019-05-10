#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(hivemind::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate hivemind;
extern crate x86_64;
extern crate bootloader;

use core::panic::PanicInfo;
use x86_64::registers::control::Cr3;
use bootloader::{BootInfo, entry_point};
use hivemind::println;

entry_point!(kernel_main);

// This is where it all begins
#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    hivemind::init(); // new


    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());


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