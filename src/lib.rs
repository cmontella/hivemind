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

use memory::FrameAllocator;
use memory::BumpAllocator;

#[no_mangle]
pub extern fn hivemind_entry(multiboot_info_address: usize) {
    // Start by clearing the screen
    vga_buffer::clear_screen();

    // Get info passed from multiboot
    let boot_info = unsafe { multiboot2::load(multiboot_info_address)};
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required.");
    let elf_sections_tag = boot_info.elf_sections_tag().expect("ELF sections tag required.");

    // Calculate kernel boundaries
    let kernel_start = elf_sections_tag.sections().map(|x| x.addr).min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|x| x.addr + x.size).max().unwrap();

    // Calculate multiboot info structure boundaries
    let multiboot_start = multiboot_info_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    /* ------- Print Debug Info ------- */
    println!("multiboot start: 0x{:x} end: 0x{:x}", multiboot_start, multiboot_end);
    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length: 0x{:x}", area.base_addr, area.length);
    }
    println!("kernel start: 0x{:x} end: 0x{:x}", kernel_start, kernel_end);
    println!("kernel sections:");
    for section in elf_sections_tag.sections() {
        println!("    address: 0x{:x}, size: 0x{:x}, flags: 0x{:x}", section.addr, section.size, section.flags);
    }

    /* ------- Test Memory Allocation ------- */
    let mut frame_allocator = memory::AreaFrameAllocator::new(kernel_start as usize, 
                                                              kernel_end as usize,
                                                              multiboot_start, 
                                                              multiboot_end, 
                                                              memory_map_tag.memory_areas());
    enable_nxe_bit();   
    enable_write_protect_bit();                                                           
    memory::remap_the_kernel(&mut frame_allocator, boot_info);
    frame_allocator.allocate_frame();

    use alloc::boxed::Box;
    let heap_test = Box::new(42);

    println!("Boot complete");

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