use x86_64::structures::idt::{Idt, ExceptionStackFrame};

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
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