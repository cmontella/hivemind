use memory::paging::entry::*;
use memory::paging::ENTRY_COUNT;
use core::ops::{Index, IndexMut};

// A page table

pub struct Tabe {
    entries: [Entry; ENTRY_COUNT];
}

impl Table {
    pub fn zero(&mut) {
        for entry in self.entries.iter_mut() {
            entry.set_unused();
        }
    }
}

// Make tables indexable through the [] syntax
// e.g. page_table[10]

impl Index<usize> for Table {
    type Output = Entry;
    fn index(&self, index: usize) -> &Entry {
        &self.entries[index]
    }
}

impl IndexMut<usize> for Table {
    fn index_mut(&mut self, index: usize) -> &mut Entry {
        &mut self.entries[index]
    }
}

