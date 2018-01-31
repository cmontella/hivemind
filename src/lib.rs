// HiveMind

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

#[macro_use]
mod macros;
mod memory;
mod interrupts;
mod drivers;
mod database;
mod arch;

use memory::FrameAllocator;
use linked_list_allocator::LockedHeap;
use raw_cpuid::CpuId;
use x86_64::instructions;
use alloc::BTreeMap;

#[no_mangle]
pub extern "C" fn hivemind_entry(multiboot_info_address: usize) {
    // Start by clearing the screen
    drivers::vga::clear_screen();

    println!("Booting HiveMind                                                   v0.1.0 alpha");
    
    // Print CPU info
    print_header("Detecting CPU");
    print_cpu_info();
    

    // Get info passed from multiboot
    let boot_info = unsafe { 
        multiboot2::load(multiboot_info_address)
    };
    enable_nxe_bit();   
    enable_write_protect_bit();  
  
    print_header("Initializing Memory");
    // Set up a guard page and map the heap pages
    let mut memory_controller = memory::init(boot_info);

    // initialize our IDT
    interrupts::init(&mut memory_controller);

    // Initialize the heap allocator
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
        database::database.lock().init();
    }

    loop { }
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] 
#[no_mangle] 
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPanic in {} at line {}:", file, line);
    println!("     {}", fmt);
    loop{}
}

fn enable_nxe_bit() {
    use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

// Enable write protection bits so we can't write into .code and .rodata
fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};

    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}

pub const HEAP_START: usize = 0o_000_001_000_000_0000;
pub const HEAP_SIZE: usize = 1000 * 1024; // 1000 KiB

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

fn print_cpu_info() {
    let cpuid = CpuId::new();

    // CPU Type
    if let Some(info) = cpuid.get_vendor_info() {
        println!("Vendor: {}\n", info.as_string());
    }

    // CPU Specifications
    if let Some(info) = cpuid.get_processor_frequency_info() {
        println!("CPU Base MHz: {}\n", info.processor_base_frequency());
        println!("CPU Base MHz: {}\n", info.processor_max_frequency());
        println!("Bus MHz: {}\n", info.bus_frequency());
    }

    // Cache Specs
    match cpuid.get_cache_parameters() {
        Some(cparams) => {
            for cache in cparams {
                let size = cache.associativity() * cache.physical_line_partitions() * cache.coherency_line_size() * cache.sets();
                println!("L{}-Cache size is {}", cache.level(), size);
            }
        },
        None => println!("No cache parameter information available"),
    }

    // CPU Features
    if let Some(info) = cpuid.get_feature_info() {
        println!("Features:");
        if info.has_fpu() { println!(" - fpu"); };
        if info.has_apic() { println!(" - apic"); };
        if info.has_acpi() { println!(" - acpi"); };
    }

    if let Some(info) = cpuid.get_extended_function_info() {
        if info.has_64bit_mode() { println!(" - 64bit"); };        
    }


}

fn print_header(header: &str) {
    println!("--------------------------------------------------------------------------------");
    println!("{}:", header);
    println!("--------------------------------------------------------------------------------");
}