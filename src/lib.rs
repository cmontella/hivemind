// # HiveMind

#![feature(lang_items)]
#![feature(unique)]
#![feature(const_fn)]
#![no_std]
#![feature(alloc)]
#![feature(global_allocator)]
#![feature(allocator_api)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(abi_x86_interrupt)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![feature(ptr_internals)]
#![feature(asm)]
#![feature(exclusive_range_pattern)]

// ## Prelude

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;
#[macro_use]
extern crate bitflags;
extern crate x86_64;
extern crate linked_list_allocator;
#[macro_use]
extern crate alloc;
#[macro_use]
extern crate once;
#[macro_use]
extern crate lazy_static;
extern crate bit_field;
extern crate raw_cpuid;
extern crate mech;

#[macro_use]
mod macros;
mod memory;
mod interrupts;
mod drivers;
mod arch;

use memory::FrameAllocator;
use linked_list_allocator::LockedHeap;
use x86_64::instructions;
use alloc::BTreeMap;
use arch::x86_64::cpu;
use mech::database::Database;
use spin::Mutex;

// ## Configurew Heap

pub const HEAP_START: usize = 0o_000_001_000_000_0000;
pub const HEAP_SIZE: usize = 1000 * 1024; // 1000 KiB
#[cfg(not(test))]
#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

lazy_static! {
  pub static ref MechDB: Mutex<Database> = Mutex::new(Database::new(100, 100, 100));
}

// ## Hivemind Entry

#[no_mangle]
pub extern "C" fn hivemind_entry(multiboot_info_address: usize) {
    // Start by clearing the screen
    drivers::vga::clear_screen();

    println!("Booting HiveMind                                                   v0.1.0 alpha");
    
    // Print CPU info
    print_header("Detecting CPU");
    cpu::print_cpu_info();
    

    // Get info passed from multiboot
    let boot_info = unsafe { 
        multiboot2::load(multiboot_info_address)
    };
    cpu::enable_nxe_bit();   
    cpu::enable_write_protect_bit();  
  
    print_header("Initializing Memory");
    // Set up a guard page and map the heap pages
    let mut memory_controller = memory::init(boot_info);

    // initialize our IDT
    interrupts::init(&mut memory_controller);

    // Initialize the heap allocator
    #[cfg(not(test))]
    unsafe {
        HEAP_ALLOCATOR.lock().init(HEAP_START, HEAP_START + HEAP_SIZE);
    }

    // invoke a breakpoint exception
    //x86_64::instructions::interrupts::int3();

    // invoke a page fault    
    /*unsafe {
        *(0xdeadbeaf as *mut u64) = 42;
    };*/

    // Invoke a stack overflow
    /*
    fn stack_overflow() {
        stack_overflow();
    }
    stack_overflow();*/

    print_header("Boot complete");

    unsafe {
        // Keyboard interrupts only
        instructions::port::outb(0x21,0xfd); 
        instructions::port::outb(0xa1,0xff);
        // Enable interrupts
        instructions::interrupts::enable();
    }

    {    
        MechDB.lock().init();
    }

    loop { }
}

#[cfg(not(test))]
#[lang = "eh_personality"] 
extern fn eh_personality() {
    ()
}
#[cfg(not(test))]
#[lang = "panic_fmt"] 
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPanic in {} at line {}:", file, line);
    println!("     {}", fmt);
    loop{}
}


fn print_header(header: &str) {
    println!("────────────────────────────────────────────────────────────────────────────────");
    println!("{}:", header);
    println!("────────────────────────────────────────────────────────────────────────────────");
}