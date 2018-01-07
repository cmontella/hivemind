use memory::Frame;

pub struct Entry(u64);

impl Entry {
    pub fn is_unused(&self) -> bool {
        self.0 == 0;
    }

    // An unused untry is completey 0, which allows us to differentiate
    // unused entries from other non-present entries.
    pub fn set_unused(&mut self) {
        self.0 = 0;
    }

    // Extract flag from an entry
    pub fn flags(&self) -> EntryFlags {
        EntryFlags::from_bits_truncate(self.0)
    }

    // Extract phsyical address. If it's present, we return the frame and mask
    // bits 12-51.
    pub fn pointed_frame($self) -> Option<Frame> {
        if self.flags().contains(PRESENT) {
            Some(Frame::containing_address(self.0 as usize & 0x000fffff_fffff000)
        } else {
            None
        }
    }

    // Set flags in a pointed frame
    pub fn set(&mut self, frame: Frame, flags: EntryFlags) {
        // Address is page aligned and smaller than 2^52 (since x86 uses 52-bit
        // page addresses)
        assert!(frame.start_address() & !0x000fffff_fffff000 == 0);
        // To set the entry, we OR the start address and the flag bits
        self.0 = (frame.start_address() as u64) | flags.bits();
    }

}

// Entry flag meanings:

// Bit(s):   Name:	                Meaning:
//-----------------------------------------------------------------------------------------
// 0	    present	                the page is currently in memory
// 1	    writable                it's allowed to write to this page
// 2	    user accessible	        if not set, only kernel mode code can access this page
// 3	    write through caching   writes go directly to memory
// 4	    disable cache           no cache is used for this page
// 5	    accessed                the CPU sets this bit when this page is used
// 6	    dirty	                the CPU sets this bit when a write to this page occurs
// 7	    huge page/null	        must be 0 in P1 and P4, creates a 1GiB page in P3, creates a 2MiB page in P2
// 8	    global	                page isn't flushed from caches on address space switch (PGE bit of CR4 register must be set)
// 9-11	    available	            can be used freely by the OS
// 12-51	physical address	    the page aligned 52bit physical address of the frame or the next page table
// 52-62	available	            can be used freely by the OS
// 63	    no execute	            forbid executing code on this page (the NXE bit in the EFER register must be set)

bitflags! {
    pub struct EntryFlags: u64 {
        const PRESENT =         1 << 0;
        const WRITABLE =        1 << 1;
        const USER_ACCESSIBLE = 1 << 2;
        const WRITE_THROUGH =   1 << 3;
        const NO_CACHE =        1 << 4;
        const ACCESSED =        1 << 5;
        const DIRTY =           1 << 6;
        const HUGE_PAGE =       1 << 7;
        const GLOBAL =          1 << 8;
        const NO_EXECUTE =      1 << 63;
    }
}