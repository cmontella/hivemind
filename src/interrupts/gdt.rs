use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::SegmentSelector;
use x86_64::PrivilegeLevel;

// # A Global Descriptor Table (GDT)

pub struct Gdt {
    table: [u64; 8],
    next_free: usize,
}

impl Gdt {
  pub fn new() -> Gdt {
      Gdt {
          table: [0; 8], // 8 entries. Theoretical max is 8192.
          next_free: 1,  // Stores the index of the next free entry. We init
                          // to 1 since the 0th entry needs to be 0 in a 
                          // valid GDT.
      }
  }

  // Load the GDT onto the processor
  pub fn load(&'static self) {
      use x86_64::instructions::tables::{DescriptorTablePointer, lgdt};
      use core::mem::size_of;

      let ptr = DescriptorTablePointer {
          base: self.table.as_ptr() as u64,
          limit: (self.table.len() * size_of::<u64>() - 1) as u16,
      };

      unsafe { lgdt(&ptr) };
  }

  // Add descriptors to the GDT
  pub fn add_entry(&mut self, entry: Descriptor) -> SegmentSelector {
    let index = match entry {
        // For an user segment we just push the u64 and remember the index
        Descriptor::UserSegment(value) => self.push(value),
        // For a system segment, we push the low and high u64 and use the index
        // of the low value.
        Descriptor::SystemSegment(value_low, value_high) => {
            let index = self.push(value_low);
            self.push(value_high);
            index
        }
    };
    SegmentSelector::new(index as u16, PrivilegeLevel::Ring0)
  }

  // Writes to the next_free entry and returns the corresponding index. If 
  // there is no free entry left, we panic since this likely indicates a 
  // programming error (we should never need to create more than two or three 
  // GDT entries for our kernel).
  fn push(&mut self, value: u64) -> usize {
      if self.next_free < self.table.len() {
          let index = self.next_free;
          self.table[index] = value;
          self.next_free += 1;
          index
      } else {
          panic!("GDT full");
      }
  }

}

// ## Descriptors

/*
There are two types of GDT entries in long mode: user and system segment 
descriptors. Descriptors for code and data segment segments are user segment 
descriptors. They contain no addresses since segments always span the complete 
address space on x86_64 (real segmentation is no longer supported). Thus, user 
segment descriptors only contain a few flags (e.g. present or user mode) and 
fit into a single u64 entry.

System descriptors such as TSS descriptors are different. They often contain a 
base address and a limit (e.g. TSS start and length) and thus need more than 64 
bits. Therefore, system segments are 128 bits. They are stored as two 
consecutive entries in the GDT.
*/

pub enum Descriptor {
    UserSegment(u64),
    SystemSegment(u64, u64),
}

impl Descriptor {
  // Create kernel mode code segments:
  pub fn kernel_code_segment() -> Descriptor {
      // We set the USER_SEGMENT bit to indicate a 64 bit user segment 
      // descriptor (otherwise the CPU expects a 128 bit system segment 
      // descriptor). The PRESENT, EXECUTABLE, and LONG_MODE bits are also 
      // needed for a 64-bit mode code segment.

      // The data segment registers ds, ss, and es are completely ignored 
      // in 64-bit mode, so we don't need any data segment descriptors in our 
      // GDT.
      let flags = USER_SEGMENT | PRESENT | EXECUTABLE | LONG_MODE;
      Descriptor::UserSegment(flags.bits())
  }

  // Creates a TSS descriptor for a given TSS
  pub fn tss_segment(tss: &'static TaskStateSegment) -> Descriptor {
    use core::mem::size_of;
    use bit_field::BitField;

    let ptr = tss as *const _ as u64;

    let mut low = PRESENT.bits();
    // base
    low.set_bits(16..40, ptr.get_bits(0..24));
    low.set_bits(56..64, ptr.get_bits(24..32));
    // limit (the `-1` in needed since the bound is inclusive)
    low.set_bits(0..16, (size_of::<TaskStateSegment>() - 1) as u64);
    // type (0b1001 = available 64-bit tss)
    low.set_bits(40..44, 0b1001);

    let mut high = 0;
    high.set_bits(0..32, ptr.get_bits(32..64));

    Descriptor::SystemSegment(low, high)
  }
}

// We only add flags that are relevant in 64-bit mode. For example, we omit the
// read/write bit, since it is completely ignored by the CPU in 64-bit mode.

bitflags! {
    struct DescriptorFlags: u64 {
        const CONFORMING    = 1 << 42;
        const EXECUTABLE    = 1 << 43;
        const USER_SEGMENT  = 1 << 44;
        const PRESENT       = 1 << 47;
        const LONG_MODE     = 1 << 53;
    }
}