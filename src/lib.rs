#![feature(lang_items)]
#![feature(unique)]
#![feature(const_fn)]
#![no_std]

extern crate rlibc;

mod vga_buffer;

#[no_mangle]
pub extern fn hivemind_entry() {

    vga_buffer::print_something();
    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}