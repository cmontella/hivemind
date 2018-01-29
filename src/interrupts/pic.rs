// # Program Interrupt Controller (PIC)

/*
The PIC is a chip on the motherboard to which all interrupts are sent.
External devices send IRQs to the PIC, which talks to the CPU, which talks
to the OS. The OS handles the IRQ, and then send a signal back to the PIC.

We need to enable interrupts, and then handle reading and writing data from 
the PIC. We also need to represent the PIC, which is actually a chain of two
PICs connected serially.

Code is adapted from here: https://wiki.osdev.org/8259_PIC
*/

// ## Standard ISA IRQs for an IBM-PC Compatible PIC

/*
IRQ	   Description
-----------------------------------------------------------
0	     Programmable Interrupt Timer Interrupt
1	     Keyboard Interrupt
2	     Cascade (used internally by the two PICs. never raised)
3	     COM2 (if enabled)
4	     COM1 (if enabled)
5	     LPT2 (if enabled)
6   	 Floppy Disk
7	     LPT1 / Unreliable "spurious" interrupt (usually)
8   	 CMOS real-time clock (if enabled)
9	     Free for peripherals / legacy SCSI / NIC
10   	 Free for peripherals / SCSI / NIC
11  	 Free for peripherals / SCSI / NIC
12	   PS2 Mouse
13  	 FPU / Coprocessor / Inter-processor
14  	 Primary ATA Hard Disk
15	   Secondary ATA Hard Disk
*/

// ## Prelude

use x86_64::instructions::port::{inb, outb};
use spin::Mutex;

// ## Some Constants

const PIC1:           u8  = 0x20;		// IO base address for primary PIC 
const PIC2:           u8  = 0xA0;		// IO base address for secondary PIC 
const PIC1_COMMAND:   u16 = PIC1 as u16;
const PIC1_DATA:      u16 = (PIC1 + 1) as u16;
const PIC2_COMMAND:   u16 = PIC2 as u16;
const PIC2_DATA:      u16 = (PIC2 + 1) as u16;
const PIC_EOI:        u8  = 0x20;

const ICW1_ICW4:      u8 = 0x01;		// ICW4 (not) needed 
const ICW1_SINGLE:    u8 = 0x02;		// Single (cascade) mode 
const ICW1_INTERVAL4: u8 = 0x04;		// Call address interval 4 (8) 
const ICW1_LEVEL:     u8 = 0x08;		// Level triggered (edge) mode 
const ICW1_INIT:      u8 = 0x10;		// Initialization - required! 
 
const ICW4_8086:      u8 = 0x01;		// 8086/88 (MCS-80/85) mode 
const ICW4_AUTO:      u8 = 0x02;		// Auto (normal) EOI 
const ICW4_BUF_SEC:   u8 = 0x08;		// Buffered mode/secondary 
const ICW4_BUF_PRIM:  u8 = 0x0C;		// Buffered mode/primary 
const ICW4_SFNM:      u8 = 0x10;		// Special fully nested (not) 

const PIC1_CMD2:      u16 = 0x20;
const PIC1_DATA2:     u8  = 0x21;
const PIC2_CMD2:      u16 = 0xA0;
const PIC2_DATA2:     u8  = 0xA1;
const PIC_READ_IRR:   u8  = 0x0a;    /* OCW3 irq ready next CMD read */
const PIC_READ_ISR:   u8  = 0x0b;   /* OCW3 irq service next CMD read */


// ## Modeling the PIC

pub struct PIC {

}

impl PIC {

  // Create a new PIC
  pub fn new() -> PIC {
    PIC{}
  }

  /*
  Tell the PIC that the interrupt is over with the End of Interrupt (EOI)
  If the IRQ came from the Primary PIC, it is sufficient to issue the 
  EOI only to the Primary PIC; however if the IRQ came from the secondary
  PIC, it is necessary to issue the command to both PIC chips.
  */
  pub fn send_end_of_interrupt(&self, irq: u8) {
    unsafe { 
      // IRQs greater than 8 came from the secondary PIC
      if irq >= 8 {
        outb(PIC2_COMMAND, PIC_EOI);
      }
      outb(PIC1_COMMAND, PIC_EOI); 
    };
  }

  /*
  The first command we will need to give the two PICs is the initialize 
  command (0x11), which makes the PIC wait for 3 extra "initialization 
  words" on the data port. These bytes give the PIC:
  - Its vector offset. (ICW2)
  - How it is wired to primary/secondary. (ICW3)
  - Gives additional information about the environment. (ICW4)
  */
  pub fn init(&self) {
    unsafe {
      // Save masks
      let pic1_mask: u8 = inb(PIC1_DATA);
      let pic2_mask: u8 = inb(PIC2_DATA);

      // Starts the initialization sequence (in cascade mode)
      outb_wait(PIC1_COMMAND, ICW1_INIT + ICW1_ICW4);  
      outb_wait(PIC2_COMMAND, ICW1_INIT + ICW1_ICW4);

      // ICW2: Primary PIC vector offset
      outb_wait(PIC1_DATA, 0x20);
      
      // ICW2: Secondary PIC vector offset
      outb_wait(PIC2_DATA, 0x28);                 
      
      // ICW3: tell Primary PIC that there is a Secondary PIC at IRQ2 (0000 0100)
      outb_wait(PIC1_DATA, 4);                       

      // ICW3: tell Secondary PIC its cascade identity (0000 0010)
      outb_wait(PIC2_DATA, 2);                       
      outb_wait(PIC1_DATA, ICW4_8086);
      outb_wait(PIC2_DATA, ICW4_8086);

      // Restore saved masks.
      outb(PIC1_DATA, pic1_mask);   
      outb(PIC2_DATA, pic2_mask);
    };
  }

  pub fn get_irq_register(&self, ocw3: u8) -> u16 {
    unsafe{
      outb(PIC1_CMD2, ocw3);
      outb(PIC2_CMD2, ocw3);
      let a = inb(PIC2_CMD2) << 7;
      let b = inb(PIC1_CMD2);
      println!("{:#b}",a);
      println!("{:#b}",b);
      
      //(inb(PIC2_CMD2) << 8) | inb(PIC1_CMD2)
      0
    }
  }

}

/*
We need a wait function to allow time for the PIC to execute the commands we
send to it. Normally we would use a timer to do this, but we need the PIC to
make one. Instead, we write some data to a safe port 0x80 and that should be 
enough time.
*/
fn outb_wait(port: u16, data: u8) {
  unsafe {
    outb(port, data);
    outb(0x80,0);
  }
}
lazy_static! {
  pub static ref pic: Mutex<PIC> = Mutex::new(PIC::new());
}