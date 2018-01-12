pub use self::area_frame_allocator::AreaFrameAllocator;
pub use self::paging::test_paging;
pub use self::paging::remap_the_kernel;
pub use self::heap_allocator::BumpAllocator;
use multiboot2::BootInformation;

mod area_frame_allocator;
mod paging;
mod heap_allocator;

pub const PAGE_SIZE: usize = 4096;

// A physical frame is identified by a monotonically increasing number, which 
// we will map to a physical memory location.

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {

    fn clone(&self) -> Frame {
        Frame { number: self.number }
    }

    fn containing_address(address: usize) -> Frame {
        Frame { number: address / PAGE_SIZE }
    }

    fn start_address(&self) -> usize {
        self.number * PAGE_SIZE
    }

    fn range_inclusive(start: Frame, end: Frame) -> FrameIter {
        FrameIter {
            start: start,
            end: end,
        }
    }
}

// We use a frame allocator to allocate frames. If we allocate a frame when
// there are none left, we get back None.

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

struct FrameIter {
    start: Frame,
    end: Frame,
}

impl Iterator for FrameIter {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        if self.start <= self.end {
            let frame = self.start.clone();
            self.start.number += 1;
            Some(frame)
        } else {
            None
        }
    }
 }

 pub fn init(boot_info: &BootInformation) {
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required.");
    let elf_sections_tag = boot_info.elf_sections_tag().expect("ELF sections tag required.");

    // Calculate kernel boundaries
    let kernel_start = elf_sections_tag.sections().filter(|s| s.is_allocated()).map(|x| x.addr).min().unwrap();
    let kernel_end = elf_sections_tag.sections().filter(|s| s.is_allocated()).map(|x| x.addr + x.size).max().unwrap();

    // Calculate multiboot info structure boundaries
    let multiboot_start = boot_info.start_address();
    let multiboot_end = boot_info.end_address();

    /* ------- Print Debug Info ------- */
    println!("multiboot start: {:#x} end: {:#x}", multiboot_start, multiboot_end);
    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: {:#x}, length: {:#x}", area.base_addr, area.length);
    }
    println!("kernel start: {:#x} end: {:#x}", kernel_start, kernel_end);
    println!("kernel sections:");
    for section in elf_sections_tag.sections() {
        println!("    address: {:#x}, size: {:#x}, flags: {:#x}", section.addr, section.size, section.flags);
    }

    /* ------- Test Memory Allocation ------- */
    let mut frame_allocator = area_frame_allocator::AreaFrameAllocator::new(kernel_start as usize, 
                                                              kernel_end as usize,
                                                              multiboot_start, 
                                                              multiboot_end, 
                                                              memory_map_tag.memory_areas());
                                                         
    paging::remap_the_kernel(&mut frame_allocator, boot_info);
 }