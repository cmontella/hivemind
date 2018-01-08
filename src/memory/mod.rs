pub use self::area_frame_allocator::AreaFrameAllocator;
pub use self::paging::test_paging;

mod area_frame_allocator;
mod paging;

pub const PAGE_SIZE: usize = 4096;

// A physical frame is identified by a monotonically increasing number, which 
// we will map to a physical memory location.

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
    fn containing_address(address: usize) -> Frame {
        Frame { number: address / PAGE_SIZE }
    }

    fn start_address(&self) -> usize {
        self.number * PAGE_SIZE
    }
}

// We use a frame allocator to allocate frames. If we allocate a frame when
// there are none left, we get back None.

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}