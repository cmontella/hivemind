#![feature(lang_items)]
#![feature(unique)]
#![feature(const_fn)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;

#[macro_use]
mod vga_buffer;
mod memory;

use memory::FrameAllocator;

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
    let mut frame_allocator = memory::AreaFrameAllocator::new(kernel_start as usize, kernel_end as usize,
                                                              multiboot_start, multiboot_end, memory_map_tag.memory_areas());
    for i in 0..3000 {
        frame_allocator.allocate_frame();
    }

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