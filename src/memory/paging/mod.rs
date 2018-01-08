use memory::{PAGE_SIZE, Frame, FrameAllocator};
pub use self::entry::*;

mod entry;
mod table;

const ENTRY_COUNT: usize = 512;  // With 512 entries at 8kb per entry, these 
                                 //total to a page size of 4096KB (4KiB).

// We map
pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

// A virtual page
pub struct Page {
    number: usize,
}

impl Page {
    pub fn containing_address(address: VirtualAddress) -> Page {
        assert!(address < 0x0000_8000_0000_0000 || address >= 0xffff_8000_0000_0000, "invalid address: 0x{:x}", address);
        Page { number: address / PAGE_SIZE }
    }

    fn start_address(&self) -> usize {
        self.number * PAGE_SIZE
    }

    fn p4_index(&self) -> usize {
        (self.number >> 27) & 0o777
    }
    fn p3_index(&self) -> usize {
        (self.number >> 18) & 0o777
    }
    fn p2_index(&self) -> usize {
        (self.number >> 9) & 0o777
    }
    fn p1_index(&self) -> usize {
        (self.number >> 0) & 0o777
    }
}

// Translate a virtual memory address to a physical memory address

pub fn translate_address(virtual_address: VirtualAddress) -> Option<PhysicalAddress> {
    let offset = virtual_address % PAGE_SIZE;
    translate_page(Page::containing_address(virtual_address)).map(|frame| frame.number * PAGE_SIZE * offset)
}

// Give a page, figure out its frame

fn translate_page(page: Page) -> Option<Frame> {
    use self::entry::HUGE_PAGE;

    // This is safe as long as we don't create any &mut references to the p4 table
    let p3 = unsafe { &*table::P4 }.next_table(page.p4_index());

    let huge_page = || {
        p3.and_then(|p3| {
            let p3_entry = &p3[page.p3_index()];
            // 1GiB page?
            if let Some(start_frame) = p3_entry.pointed_frame() {
                if p3_entry.flags().contains(HUGE_PAGE) {
                    // address must be 1GiB aligned
                    assert!(start_frame.number % (ENTRY_COUNT * ENTRY_COUNT) == 0);
                    return Some(Frame {
                        number: start_frame.number + page.p2_index() *
                                ENTRY_COUNT + page.p1_index(),
                    });
                }
            }
            if let Some(p2) = p3.next_table(page.p3_index()) {
                let p2_entry = &p2[page.p2_index()];
                // 2MiB page?
                if let Some(start_frame) = p2_entry.pointed_frame() {
                    if p2_entry.flags().contains(HUGE_PAGE) {
                        // address must be 2MiB aligned
                        assert!(start_frame.number % ENTRY_COUNT == 0);
                        return Some(Frame {
                            number: start_frame.number + page.p1_index()
                        });
                    }
                }
            }
            None
        })
    };

    p3.and_then(|p3| p3.next_table(page.p3_index()))
      .and_then(|p2| p2.next_table(page.p2_index()))
      .and_then(|p1| p1[page.p1_index()].pointed_frame())
      .or_else(huge_page)
}

// Modifies page tables to map a page to a frame

pub fn map_to<A>(page: Page, frame: Frame, flags: , allocator: &mut A) 
    where A: FrameAllocator {

    let p4 = unsafe { &mut *P4 };
    let mut p3 = p4.next_table_create(page.p4_index(), allocator);
    let mut p2 = p3.next_table_create(page.p3_index(), allocator);
    let mut p1 = p2.next_table_create(page.p2_index(), allocator);

    assert!(p1[page.p1_index()].is_unused());
    p1[page.p1_index()].set(frame, flags | PRESENT);
}

// Return the next table, if it exists, or create a new one

pub fn next_table_create<A>(&mut self, index: usize, allocator: &mut A) -> &mut Table<L::NextLevel> 
    where A: FrameAllocator {

    if self.next_table(index).is_none() {
        assert!(!self.entries[index].flags().contains(HUGE_PAGE),
                "mapping code does not support huge pages");
        let frame = allocator.allocate_frame().expect("no frames available");
        self.entries[index].set(frame, PRESENT | WRITABLE);
        self.next_table_mut(index).unwrap().zero();
    }
    self.next_table_mut(index).unwrap()

}