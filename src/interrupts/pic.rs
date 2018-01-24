use x86_64::instructions::port::{inb, outb};

// # Program Interrupt Chip (PIC)

// The PIC is a chip on the motherboard to which all interrupts are sent.
// External devices send IRQs to the PIC, which talks to the CPU, which talks
// to the OS. The OS handles the IRQ, and then send a signal back to the PIC.

// We need to enable interrupts, and then handle reading and writing data from 
// the PIC. We also need to represent the PIC, which is actually a chain of two
// PICs connected serially.

// Code is adapted from here: https://wiki.osdev.org/8259_PIC

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
 
const ICW4_8086:       u8 = 0x01;		// 8086/88 (MCS-80/85) mode 
const ICW4_AUTO:       u8 = 0x02;		// Auto (normal) EOI 
const ICW4_BUF_SEC:    u8 = 0x08;		// Buffered mode/secondary 
const ICW4_BUF_PRIM:   u8 = 0x0C;		// Buffered mode/primary 
const ICW4_SFNM:       u8 = 0x10;		// Special fully nested (not) 

struct PIC {

}

impl PIC {

  // Tell the PIC that the interrupt is over with the End of Interrupt (EOI)
  // If the IRQ came from the Primary PIC, it is sufficient to issue the 
  // EOI only to the Primary PIC; however if the IRQ came from the secondary
  //  PIC, it is necessary to issue the command to both PIC chips.
  pub fn send_end_of_interrupt(irq: u8) {
    unsafe { 
      // IRQs greater than 8 came from the secondary PIC
      if irq >= 8 {
        outb(PIC2_COMMAND, PIC_EOI); };
      }
      outb(PIC1_COMMAND, PIC_EOI); 
    };
  }

  // The first command we will need to give the two PICs is the initialize 
  // command (0x11), which makes the PIC wait for 3 extra "initialization 
  // words" on the data port. These bytes give the PIC:
  // - Its vector offset. (ICW2)
  // - How it is wired to primary/secondary. (ICW3)
  // - Gives additional information about the environment. (ICW4)
  pub fn init(offset1: u8, offset2: u8) {
    unsafe {
      // Save masks
      let pic1_mask: u8 = inb(PIC1_DATA);
      let pic2_mask: u8 = inb(PIC2_DATA);

      // Starts the initialization sequence (in cascade mode)
      outb(PIC1_COMMAND, ICW1_INIT + ICW1_ICW4);  
      io_wait();
      outb(PIC2_COMMAND, ICW1_INIT + ICW1_ICW4);
      io_wait();

      // ICW2: Primary PIC vector offset
      outb(PIC1_DATA, offset1);                 
      io_wait();
      
      // ICW2: Secondary PIC vector offset
      outb(PIC2_DATA, offset2);                 
      io_wait();
      
      // ICW3: tell Primary PIC that there is a Secondary PIC at IRQ2 (0000 0100)
      outb(PIC1_DATA, 4);                       
      io_wait();

      // ICW3: tell Secondary PIC its cascade identity (0000 0010)
      outb(PIC2_DATA, 2);                       
      io_wait();
      outb(PIC1_DATA, ICW4_8086);
      io_wait();
      outb(PIC2_DATA, ICW4_8086);
      io_wait();

      // Restore saved masks.
      outb(PIC1_DATA, pic1_mask);   
      outb(PIC2_DATA, pic2_mask);
    };
  }

}

// We need a wait function to allow time for the PIC to execute the commands we
// send to it. Normally we would use a timer to do this, but we need the PIC to
// make one. Instead, we write some data to a safe port 0x80 and that should be 
// enough time.
fn io_wait() {
  unsafe {
    outb(0x80,0);
  }
}