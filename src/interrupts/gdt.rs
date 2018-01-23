// A Global Descriptor Table (GDT)

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
}

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
      let flags = USER_SEGMENT | PRESENT | EXECUTABLE | LONG_MODE;
      Descriptor::UserSegment(flags.bits())
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