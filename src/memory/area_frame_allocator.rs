use memory::{Frame, FrameAllocator};
use multiboot2::{MemoryAreaIter, MemoryArea};

// The Area Frame Allocator is initialized with boundaries for the kernel
// and the multiboot section. We use these boundaries to avoid allocating
// any pages in these ranges. It also tracks the next free frame, and the
// current area.

pub struct AreaFrameAllocator {
    next_free_frame: Frame,
    current_area: Option<&'static MemoryArea>,
    areas: MemoryAreaIter,
    kernel_start: Frame,
    kernel_end: Frame,
    multiboot_start: Frame,
    multiboot_end: Frame,
}

impl AreaFrameAllocator {
    
    pub fn new(kernel_start: usize, kernel_end: usize, 
               multiboot_start: usize, multiboot_end: usize, 
               memory_areas: MemoryAreaIter) -> AreaFrameAllocator {
        let mut allocator = AreaFrameAllocator {
            next_free_frame: Frame::containing_address(0),
            current_area: None,
            areas: memory_areas,
            kernel_start: Frame::containing_address(kernel_start),
            kernel_end: Frame::containing_address(kernel_end),
            multiboot_start: Frame::containing_address(multiboot_start),
            multiboot_end: Frame::containing_address(multiboot_end),
        };
        allocator.choose_next_area();
        allocator
    }

    // We choose the next area by looking at all of the areas, taking all those that are
    // beyond the boundary of the next free frame, and then choosing the one with the
    // smallest page number. If none exists, this returns None.

    fn choose_next_area(&mut self) {
        self.current_area = self.areas.clone().filter(|area| {
            let address = area.base_addr + area.length - 1;
            Frame::containing_address(address as usize) >= self.next_free_frame
        }).min_by_key(|area| area.base_addr);

        if let Some(area) = self.current_area {
            let start_frame = Frame::containing_address(area.base_addr as usize);
            if self.next_free_frame < start_frame {
                self.next_free_frame = start_frame;
            }
        }
    }
}

impl FrameAllocator for AreaFrameAllocator {

    fn allocate_frame(&mut self) -> Option<Frame> {
        // If there is a free area, return it
        if let Some(area) = self.current_area {
            // Clone the frame to return it if it's free
            let frame = Frame { number: self.next_free_frame.number };

            // the last frame of the current area
            let current_area_last_frame = {
                let address = area.base_addr + area.length - 1;
                Frame::containing_address(address as usize)
            };

            // If all frames of the current area are used, switch to the next area
            if frame > current_area_last_frame {
                self.choose_next_area();
            // If the frame is used by the kernel, move to the next frame after the kernel
            } else if frame >= self.kernel_start && frame <= self.kernel_end {
                println!("Kernel");
                self.next_free_frame = Frame { number: self.kernel_end.number + 1 };
            // If the frame is used by multiboot, move to the next frame after multiboot                
            } else if frame >= self.multiboot_start && frame <= self.multiboot_end {
                println!("Multiboot");
                self.next_free_frame = Frame { number: self.multiboot_end.number + 1 };
            // Else, the frame is unused and we can use it
            } else {
                self.next_free_frame.number += 1;
                return Some(frame);
            }
            self.allocate_frame()

        // If there are no more free areas, return None
        } else {
            println!("No more free areas!!!");
            None
        }
    }

    // TODO we'll implement this later
    fn deallocate_frame(&mut self, frame: Frame) {
            unimplemented!();
    }
}