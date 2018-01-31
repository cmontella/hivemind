// # Real Time Clock (RTC)

// ## Prelude

use x86_64::instructions::port::{inb, outb};
use x86_64::instructions::interrupts;
use spin::Mutex;

pub struct RTC {
  pub val: u8,
}

impl RTC {

  pub fn new() -> RTC {
    RTC {
      val: 0,
    }
  }

  pub fn init(&self) {
    unsafe {
        outb(0x70, 0x8B);		      // select register B, and disable NMI
        let prev = inb(0x71);	    // read the current value of register B
        outb(0x70, 0x8B);		      // set the index again (a read will reset the index to register D)
        outb(0x71, prev | 0x40);	// write the previous value ORed with 0x40. This turns on bit 6 of register B
    }
  }

  pub fn read_byte(&mut self) {
    unsafe {
        interrupts::disable();
        outb(0x70, 0x0C);	  // select register C
        self.val = inb(0x71);		      // just throw away contents
        outb(0x20,0x20);    // Send Ack to PIC 1
        outb(0xA0,0x20);    // Send ACK to PIC 2
        interrupts::enable();
    }
  }
}

lazy_static! {
  pub static ref rtc: Mutex<RTC> = Mutex::new(RTC::new());
}


