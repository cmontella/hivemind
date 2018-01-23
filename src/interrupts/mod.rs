use x86_64::structures::idt::{Idt, ExceptionStackFrame};
use memory::MemoryController;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtualAddress;
use spin::Once;

mod gdt;

// The zeroth IST entry is the double fault stack. Any other one would work,
// but this is fine.
const DOUBLE_FAULT_IST_INDEX: usize = 0;

static TSS: Once<TaskStateSegment> = Once::new();
static GDT: Once<gdt::Gdt> = Once::new();

// Interrupt Descriptor Table (IDT)
// The IDT hold pointers to handler functions for various exceptions and interrupts.

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX as u16);
        }
        idt
    };
}

// Initialize the IDT

pub fn init(memory_controller: &mut MemoryController) {
    use x86_64::structures::gdt::SegmentSelector;
    use x86_64::instructions::segmentation::set_cs;
    use x86_64::instructions::tables::load_tss;

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
        let mut gdt = gdt::Gdt::new();
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

    IDT.load();
}

// ## Exception Handlers

// Breakpoints
// Breakpoints are set by the user to aid in debugging. 

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
  println!("Breakpoint:\n{:#?}", stack_frame);
}

// Double faults
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