use x86_64::structures::idt::{Idt, ExceptionStackFrame};

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt
    };
}

pub fn init() {
    IDT.load();
}

// Handle breakpoints

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
  println!("A breakpoint exception occurred\n{:#?}", stack_frame);
}

// Handle double faults. Double faults can only occur in specific combinations of exceptions

/*
In the case of:
Divide-by-zero,
Invalid TSS,
Segment Not Present,
Stack-Segment Fault,
General Protection Fault	

A Double double fault occurs if:
Invalid TSS,
Segment Not Present,
Stack-Segment Fault,
General Protection Fault

In the case of:
Page Fault	

A Double double fault occurs if:
Page Fault,
Invalid TSS,
Segment Not Present,
Stack-Segment Fault,
General Protection Fault
*/

extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
    println!("\nEXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop {}
}