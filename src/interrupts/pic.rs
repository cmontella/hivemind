use x86_64::instructions::port;

// # Program Interrupt Chip (PIC)

// The PIC is a chip on the motherboard to which all interrupts are sent.
// External devices send IRQs to the PIC, which talks to the CPU, which talks
// to the OS. The OS handles the IRQ, and then send a signal back to the PIC.

// We need to enable interrupts, and then handle reading and writing data from 
// the PIC. We also need to represent the PIC, which is actually a chain of two
// PICs connected serially.

// For more, see here: https://wiki.osdev.org/8259_PIC

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


const PIC1: u8 = 0x20;		/* IO base address for primary PIC */
const PIC2: u8 = 0xA0;		/* IO base address for secondary PIC */
const PIC1_COMMAND: u8 = PIC1;
const PIC1_DATA: u8 = (PIC1 + 1);
const PIC2_COMMAND: u8 = PIC2;
const PIC2_DATA: u8 = (PIC2 + 1);
const PIC_EOI: u8 = 0x20;

struct PIC {

}

impl PIC {

  // Tell the PIC that the interrupt is over with the End of Interrupt (EOI)
  // If the IRQ came from the Primary PIC, it is sufficient to issue the 
  // EOI only to the Primary PIC; however if the IRQ came from the secondary
  //  PIC, it is necessary to issue the command to both PIC chips.
  pub fn send_end_of_interrupt(irq: u8) {
    // IRQs greater than 8 came from the secondary PIC
    if irq >= 8 {
      unsafe { port::outb(PIC2_COMMAND as u16, PIC_EOI); };
    }
    unsafe { port::outb(PIC_COMMAND as u16, PIC_EOI); };
  }

}