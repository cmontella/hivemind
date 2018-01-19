//HiveMind

#![feature(lang_items)]
#![feature(unique)]
#![feature(const_fn)]
#![no_std]
#![feature(alloc)]
#![feature(global_allocator)]
#![feature(allocator_api)]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;
#[macro_use]
extern crate bitflags;
extern crate x86_64;

#[macro_use]
mod vga_buffer;
mod memory;
#[macro_use]
extern crate alloc;
#[macro_use]
extern crate once;

use memory::FrameAllocator;
use memory::BumpAllocator;

#[no_mangle]
pub extern fn hivemind_entry(multiboot_info_address: usize) {
    // Start by clearing the screen
    vga_buffer::clear_screen();

    println!("Loading Hivemind v0.1.0 ...");

    // Get info passed from multiboot
    let boot_info = unsafe { multiboot2::load(multiboot_info_address)};
    memory::init(boot_info);
    //enable_nxe_bit();   
    //enable_write_protect_bit();  
    
    //use alloc::boxed::Box;
    //let heap_test = Box::new(42);

    println!("Boot complete.");

    loop{}
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
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

#[global_allocator]
static HEAP_ALLOCATOR: BumpAllocator = BumpAllocator::new(HEAP_START,
    HEAP_START + HEAP_SIZE);