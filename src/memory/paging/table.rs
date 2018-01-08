use memory::paging::entry::*;
use memory::paging::ENTRY_COUNT;
use core::ops::{Index, IndexMut};
use core::marker::PhantomData;

// Table levels for page tables. These allow us to customize the page tables
// depending on their level.

pub trait TableLevel{}

pub enum Level4 {}
pub enum Level3 {}
pub enum Level2 {}
pub enum Level1 {}

impl TableLevel for Level4 {}
impl TableLevel for Level3 {}
impl TableLevel for Level2 {}
impl TableLevel for Level1 {}

pub trait HierarchicalLevel: TableLevel {
    type NextLevel: TableLevel;
}

impl HierarchicalLevel for Level4 {
    type NextLevel = Level3;
}

impl HierarchicalLevel for Level3 {
    type NextLevel = Level2;
}
impl HierarchicalLevel for Level2 {
    type NextLevel = Level1;
}

// Level 1 tables don't get this trait, since we want to restrict their
// ability to access next_table().

// A page table

pub struct Table<L: TableLevel> {
    entries: [Entry; ENTRY_COUNT],
    level: PhantomData<L>,
}

impl<L> Table<L> where L: TableLevel {
    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.set_unused();
        }
    }
}

impl<L> Table<L> where L: HierarchicalLevel {
    // The next table addres is valid only if the next table is present and is
    // not huge. This will panic if the index is out of bounds, which we want,
    // since accessing this index indicates a bug.

    fn next_table_address(&self, index: usize) -> Option<usize> {
        let entry_flags = self[index].flags();
        if entry_flags.contains(PRESENT) && !entry_flags.contains(HUGE_PAGE) {
            let table_address = self as *const _ as usize;
            Some((table_address << 9) | (index << 12))
        } else {
            None
        }
    }

    // A page table owns all of its subtables

    pub fn next_table<'a>(&'a self, index: usize) -> Option<&'a Table<L::NextLevel>> {
        self.next_table_address(index).map(|address| unsafe { &*(address as *const _)})
    }

    pub fn next_table_mut<'a>(&'a self, index: usize) -> Option<&'a mut Table<L::NextLevel>> {
        self.next_table_address(index).map(|address| unsafe { &mut *(address as *mut _)})
    }

    // Return the next table, if it exists, or create a new one

    pub fn next_table_create<A>(&mut self, index: usize, allocator: &mut A) -> &mut Table<L::NextLevel> 
        where A: FrameAllocator {

        if self.next_table(index).is_none() {
            assert!(!self.entries[index].flags().contains(HUGE_PAGE),"mapping code does not support huge pages");
            let frame = allocator.allocate_frame().expect("no frames available");
            self.entries[index].set(frame, PRESENT | WRITABLE);
            self.next_table_mut(index).unwrap().zero();
        }
        self.next_table_mut(index).unwrap()
    }

}

// Make tables indexable through the [] syntax
// e.g. page_table[10]

impl<L> Index<usize> for Table<L> where L: TableLevel {
    type Output = Entry;
    fn index(&self, index: usize) -> &Entry {
        &self.entries[index]
    }
}

impl<L> IndexMut<usize> for Table<L> where L: TableLevel {
    fn index_mut(&mut self, index: usize) -> &mut Entry {
        &mut self.entries[index]
    }
}

// The 511th entry of the active P4 table must always be mapped to the active P4 table itself.
pub const P4: *mut Table<Level4> = 0xffffffff_fffff000 as *mut _;