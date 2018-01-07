use memory::PAGE_SIZE

mod entry;
mod table;

const ENTRY_COUNT: usize = 512;  // With 512 entries at 8kb per entry, these 
                                 //total to a page size of 4096KB (4KiB).

// We map
pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

// A virtual page
pub struct Page {
    number: usize;
}