use x86_64::structures::idt::{Idt, ExceptionStackFrame};

pub fn init() {
  let mut idt = Idt::new();
  idt.breakpoint.set_handler_fn(breakpoint_handler);
}

// Handle breakpoints

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
  println!("A breakpoint exception occurred\n{:#?}", stack_frame);
}