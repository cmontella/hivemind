// # Real Time Clock (RTC)

// ## Prelude

use x86_64::instructions::port::{inb, outb};
use x86_64::instructions::interrupts;


pub fn init() {
  unsafe {
      interrupts::disable();
      outb(0x70, 0x0C);	  // select register C
      inb(0x71);		      // just throw away contents
      outb(0x20,0x20);    // Send Ack to PIC 1
      outb(0xA0,0x20);    // Send ACK to PIC 2
      interrupts::enable();
  }
}