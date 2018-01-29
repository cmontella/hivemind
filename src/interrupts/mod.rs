// # Interrupts

use x86_64::structures::idt::{Idt, ExceptionStackFrame, PageFaultErrorCode};
use memory::MemoryController;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtualAddress;
use spin::Once;
use x86_64::instructions::port::{inb, outb};
use x86_64::instructions::interrupts;
use drivers::{keyboard, rtc};

mod gdt;
mod pic;

// The zeroth IST entry is the double fault stack. Any other one would work,
// but this is fine.

const DOUBLE_FAULT_IST_INDEX: usize = 0;

static TSS: Once<TaskStateSegment> = Once::new();
static GDT: Once<gdt::GDT> = Once::new();
static PIC: Once<pic::PIC> = Once::new();

// ## Create and Initialize the Interrupt Descriptor Table (IDT)

// The IDT hold pointers to handler functions for various exceptions and interrupts.

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.interrupts[0].set_handler_fn(pit_handler);
        idt.interrupts[1].set_handler_fn(keyboard_handler);
        idt.interrupts[8].set_handler_fn(rtc_handler);
        //println!("Set interrupt handlers");
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX as u16);
        }
        idt
    };
}

// Initialize the Interrupt Descriptor Table (IDT)

pub fn init(memory_controller: &mut MemoryController) {
    use x86_64::structures::gdt::SegmentSelector;
    use x86_64::instructions::segmentation::set_cs;
    use x86_64::instructions::tables::load_tss;

    // We need to create a clean stack for the CPU to switch to in the
    // event of a double fault. Here we go...

    // We allocate one page (4096 bytes) for our double fault handler.
    let handler_pages = 1; 
    let double_fault_stack = memory_controller.alloc_stack(handler_pages)
        .expect("Could not allocate double fault stack.");

    // Create a Task State Segment (TSS) that contains our double fault stack
    // in its interrupt stack table.
    let tss = TSS.call_once(|| {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX] = VirtualAddress(
            double_fault_stack.top()); // Load to top and it grows down.
        tss
    });

    // Load TSS into GDT
    let mut code_selector = SegmentSelector(0);
    let mut tss_selector = SegmentSelector(0);
    
    let gdt = GDT.call_once(|| {
        let mut gdt = gdt::GDT::new();
        code_selector = gdt.add_entry(gdt::Descriptor::kernel_code_segment());
        tss_selector = gdt.add_entry(gdt::Descriptor::tss_segment(&tss));
        gdt
    });
    gdt.load();

    unsafe {
        // Reload code segment register
        set_cs(code_selector);
        // Load TSS
        load_tss(tss_selector);
    }

    // Load the IDT into the CPU
    IDT.load();

    // Initialize PIC
    let pic = PIC.call_once(||{
        let mut pic = pic::PIC::new();
        pic.init();
        pic
    });

    println!("Enabling RTC");
    // Turn on RTC
    unsafe {
        outb(0x70, 0x8B);		    // select register B, and disable NMI
        let prev = inb(0x71);	    // read the current value of register B
        outb(0x70, 0x8B);		    // set the index again (a read will reset the index to register D)
        outb(0x71, prev | 0x40);	// write the previous value ORed with 0x40. This turns on bit 6 of register B
    }
    println!("RTC Enabled");


}

// ## Exception Handlers

// ### Breakpoints

// Breakpoints are set by the user to aid in debugging. 

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
  println!("Breakpoint:\n{:#?}", stack_frame);
}

// ### Page Faults
/*
Page faults occur when memory is accessed in an inappropriate way. An error 
code is returned with various flags that can be set:

- PROTECTION_VIOLATION - the page fault was caused by a page-protection violation, else the page fault was caused by a not-present page.
- CAUSED_BY_WRITE - If this flag is set, the memory access that caused the page fault was a write.
- USER_MODE - If this flag is set, an access in user mode (CPL=3) caused the page fault.
- MALFORMED_TABLE - If this flag is set, the page fault is a result of the processor reading a 1 from a reserved field within a page-translation-table entry.
- INSTRUCTION_FETCH - If this flag is set, it indicates that the access that caused the page fault was an instruction fetch.
*/

extern "x86-interrupt" fn page_fault_handler(stack_frame: &mut ExceptionStackFrame, error_code: PageFaultErrorCode) {
  println!("Page Fault: {:?}\n{:#?}", error_code, stack_frame);
}

// ### Double faults

// Double faults can only occur in specific combinations of exceptions:

/*-----------------------------------------------------------------------------
-- In the case of: --
Divide-by-zero,
Invalid TSS,
Segment Not Present,
Stack-Segment Fault,
General Protection Fault	

-- A Double double fault occurs if: --
Invalid TSS,
Segment Not Present,
Stack-Segment Fault,
General Protection Fault
------------------------------------------------------------------------------|
-- In the case of: --
Page Fault	

-- A Double double fault occurs if: --
Page Fault,
Invalid TSS,
Segment Not Present,
Stack-Segment Fault,
General Protection Fault
-----------------------------------------------------------------------------*/

extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
    println!("Double Fault:\n{:#?}", stack_frame);
    loop {}
}

// ## Interrupts

// Interrupts occur when an external device wants to gain execution time. It
// will send an Interrupt Request (IRQ) which will be handled by the IDT in a
// similar fashion to handling exceptions.

// ### Programmable Interrupt Timer (PIT)

const PIT_DATA0: u8 = 0x40;  // Channel 0 data port (read/write)
const PIT_DATA1: u8 = 0x41;  // Channel 1 data port (read/write)
const PIT_DATA2: u8 = 0x42;  // Channel 2 data port (read/write)
const PIT_CMD:   u8 = 0x43;  // Mode/Command register (write only, a read is ignored)

extern "x86-interrupt" fn pit_handler(stack_frame: &mut ExceptionStackFrame) {
    //println!("PIT:\n{:#?}", stack_frame);
    unsafe {
        outb(0x20,0x20);
    }
}

// ### Keyboard

extern "x86-interrupt" fn keyboard_handler(stack_frame: &mut ExceptionStackFrame) {
    unsafe {
        interrupts::disable();
    }
    unsafe { keyboard::read_byte() };
    unsafe {
        outb(0x20, 0x20);     
        interrupts::enable();
    }
}

// ### Real Time Clock (RTC)

extern "x86-interrupt" fn rtc_handler(stack_frame: &mut ExceptionStackFrame) {
    rtc::init();
}