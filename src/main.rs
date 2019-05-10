#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(hivemind::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate hivemind;
extern crate bootloader;
extern crate x86_64;
#[macro_use]
extern crate alloc;

use hivemind::println;
use hivemind::memory::{self, BootInfoFrameAllocator};
use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use alloc::boxed::Box;
use alloc::vec::Vec;
use x86_64::{structures::paging::Page, VirtAddr};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {

    println!("Hello World{}", "!");
    hivemind::init();

    let mut mapper = unsafe { memory::init(boot_info.physical_memory_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // map a previously unmapped page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    hivemind::hlt_loop();
}

/// This function is called on panic.
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