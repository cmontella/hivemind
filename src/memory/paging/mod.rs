use memory::{PAGE_SIZE, Frame, FrameAllocator};
use self::table::{Table, Level4};
use core::ptr::Unique;
pub use self::entry::*;

mod entry;
mod table;
mod temporary_page;

const ENTRY_COUNT: usize = 512;  // With 512 entries at 8kb per entry, these 
                                 //total to a page size of 4096KB (4KiB).

// We map
pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

// A virtual page
#[derive(Debug, Clone, Copy)]
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

// The recursively mapped P4 table is owned by a ActivePageTable struct.

pub struct ActivePageTable {
    p4: Unique<Table<Level4>>,
}

// ActivePageTable is unique -- there can only ever be one instance.

impl ActivePageTable {

    pub unsafe fn new() -> ActivePageTable {
        ActivePageTable {
            p4: Unique::new_unchecked(table::P4),
        }
    }

    // We can get references to the p4 table owned by the active page table.
    // From there, we can get references to P3 - P1.

    fn p4(&self) -> &Table<Level4> {
        unsafe { self.p4.as_ref() }
    }

    fn p4_mut(&mut self) -> &mut Table<Level4> {
        unsafe { self.p4.as_mut() }
    }

    // Translates a virtual to the corresponding physical address.
    // Returns `None` if the address is not mapped.

    pub fn translate(&self, virtual_address: VirtualAddress) -> Option<PhysicalAddress> {
        let offset = virtual_address % PAGE_SIZE;
        self.translate_page(Page::containing_address(virtual_address)).map(|frame| frame.number * PAGE_SIZE * offset)
    }
    
    // Give a page, figure out its frame

    fn translate_page(&self, page: Page) -> Option<Frame> {
        use self::entry::HUGE_PAGE;

        // This is safe as long as we don't create any &mut references to the p4 table
        let p3 = self.p4().next_table(page.p4_index());

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

    // Maps the page to the frame with the provided flags.
    // The `PRESENT` flag is added by default. Needs a
    // `FrameAllocator` as it might need to create new page tables.// Modifies page tables to map a page to a frame
    pub fn map_to<A>(&mut self, page: Page, frame: Frame, flags: EntryFlags, allocator: &mut A) 
        where A: FrameAllocator 
    {
        let p4 = self.p4_mut();
        let mut p3 = p4.next_table_create(page.p4_index(), allocator);
        let mut p2 = p3.next_table_create(page.p3_index(), allocator);
        let mut p1 = p2.next_table_create(page.p2_index(), allocator);

        assert!(p1[page.p1_index()].is_unused());
        p1[page.p1_index()].set(frame, flags | PRESENT);
    }

    // Maps the page to some free frame with the provided flags.
    // The free frame is allocated from the given `FrameAllocator`.
    pub fn map<A>(&mut self, page: Page, flags: EntryFlags, allocator: &mut A)
        where A: FrameAllocator
    {
        let frame = allocator.allocate_frame().expect("out of memory");
        self.map_to(page, frame, flags, allocator)
    }

    // Identity map the the given frame with the provided flags.   
    // The `FrameAllocator` is used to create new page tables if needed.
    pub fn identity_map<A>(&mut self, frame: Frame, flags: EntryFlags, allocator: &mut A)
        where A: FrameAllocator
    {
        let page = Page::containing_address(frame.start_address());
        self.map_to(page, frame, flags, allocator)
    }
    
    // Unmaps the given page and adds all freed frames to the given
    // `FrameAllocator`.
    fn unmap<A>(&mut self, page: Page, allocator: &mut A)
        where A: FrameAllocator
    {
        assert!(self.translate(page.start_address()).is_some());

        let p1 = self.p4_mut()
                    .next_table_mut(page.p4_index())
                    .and_then(|p3| p3.next_table_mut(page.p3_index()))
                    .and_then(|p2| p2.next_table_mut(page.p2_index()))
                    .expect("mapping code does not support huge pages");
        let frame = p1[page.p1_index()].pointed_frame().unwrap();
        p1[page.p1_index()].set_unused();
        
        use x86_64::instructions::tlb;
        use x86_64::VirtualAddress;
        tlb::flush(VirtualAddress(page.start_address()));

        // TODO free p(1,2,3) table if empty
        //allocator.deallocate_frame(frame);
    }
}

pub fn test_paging<A>(allocator: &mut A)
    where A: FrameAllocator
{
    let mut page_table = unsafe { ActivePageTable::new() };

    // test it
    let addr = 42 * 512 * 512 * 4096; // 42th P3 entry
    let page = Page::containing_address(addr);
    let frame = allocator.allocate_frame().expect("no more frames");
    println!("None = {:?}, map to {:?}",
            page_table.translate(addr),
            frame);
    page_table.map_to(page, frame, EntryFlags::empty(), allocator);
    println!("Some = {:?}", page_table.translate(addr));
    println!("next free frame: {:?}", allocator.allocate_frame());
    println!("{:#x}", unsafe {
        *(Page::containing_address(addr).start_address() as *const u64)
    });

    page_table.unmap(Page::containing_address(addr), allocator);
    println!("None = {:?}", page_table.translate(addr));
}

// We need a way to work on Inactive Page Tables

pub struct InactivePageTable {
    p4_frame: Frame,
}

impl InactivePageTable {
    pub fn new(frame: Frame) -> InactivePageTable {
        // TODO zero and recursive map the frame
        InactivePageTable { p4_frame: frame }
    }
}