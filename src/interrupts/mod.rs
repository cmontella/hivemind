// # Interrupts

use x86_64::structures::idt::{Idt, ExceptionStackFrame, PageFaultErrorCode};
use memory::MemoryController;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtualAddress;
use spin::Once;
use x86_64::instructions::port::{inb, outb};
use vga_buffer::{SCREEN_WRITER, print};

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
        pic.get_irq_register(0x0a);
        pic
    });

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
    {}
}

// ### Keyboard

// #### Code Page 437

#[derive(Debug)]
pub enum KeyCode {
    //Null, LightSmile, DarkSmile, Heart, Diamond, CLub, Bullet, BulletBackground, Circle, CircleBackground, Mars, Venus, EigthNote, SixteenthNote, Sun,
    //RightTriangle, LeftTriangle, DoubleArrowVertical, DoubleExclaimation, Pilcrow, Section, Bar, DoubleArrowBottom, UpArrow, DownArrow, RightArrow, LeftArrow, RightAngle, DoubleArrowHorz, UpTriangle, DownTriangle,
    Space = 32, Exclaimation = 33, Quote = 34, Hash = 35, Dollar = 36, Percent = 37, Ampersand = 38, Apostrophe = 39, LeftParenthesis = 40, RightParenthesis = 41, Asterisk = 42, Plus = 43, Comma = 44, Minus = 45, FullStop = 46, Slash = 47,
    Zero = 48, One = 49, Two = 50, Three = 51,/* Four, Five, Six, Seven, Eight, Nine, Colon, Semicolon, LeftChevron, Equal, RightChevron,  Question,
    At, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, LeftBracket, BackSlash, RightBracket, Caret, Underscore,
    Grave, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, LeftBrace, Pipe, RightBrace, Tilde, House,
    C_Cedilla, u_Umlaut, e_Acute,  a_Circumflex, a_Umlaut, a_Grave, a_Volle, c_Cedilla, e_Circumflex, e_Umlaut, e_Grave, i_Umlaut, i_Circumflex, i_Grave, A_Umlaut, A_Volle, 
    E_Acute, ae, AE, o_Circumflex, o_Umlaut, o_Grave, u_Circumflex, u_Grave, y_Umlaut, O_Umlaut, U_Umlaut, Cents, PoundSterling, Yen, Pesta, ScriptF,
    a_Acute, i_Acute, o_acute, u_acute, n_Tilde, N_Tilde, a_Ordinal, o_Ordinal, InvertedQuestion, LeftNegation, RightNegation, Half, Quarter, InvertedExclaimation, LeftAngleQuotes, RightAngleQuotes, 
    LightBlock, MediumBlock, BoxDrawing179, BoxDrawing180, BoxDrawing181, BoxDrawing182, BoxDrawing183, BoxDrawing184, BoxDrawing185, BoxDrawing186, BoxDrawing187, BoxDrawing188, BoxDrawing189, BoxDrawing190, BoxDrawing191,
    BoxDrawing192, BoxDrawing193, BoxDrawing194, BoxDrawing195, BoxDrawing196, BoxDrawing197, BoxDrawing198, BoxDrawing199, BoxDrawing200, BoxDrawing201, BoxDrawing202, BoxDrawing203, BoxDrawing204, BoxDrawing205, BoxDrawing206, BoxDrawing207,
    BoxDrawing208, BoxDrawing209, BoxDrawing210, BoxDrawing211, BoxDrawing212, BoxDrawing213, BoxDrawing214, BoxDrawing215, BoxDrawing216, BoxDrawing217, BoxDrawing218, SolidBlock, BoxDrawing220, BoxDrawing221, BoxDrawing222, BoxDrawing223,
    alpha, beta, Gamma, pi, Sigma, sigma, mu, tau, Phi, Theta, omega, delta, Lemniscate, phi, epsilon, Intersection, TripleBar, PlusMinus, GreaterThanEqual, LessThanEqual, IntegrateTop, IntegrateBottom, Divide, Approximate, Degree, Bullet2, Interrupt, SquareRoot, NthPower, Squared, Square, NonBreakingSpace,
    Enter,
    Escape,
    BackSpace,
    F1,F2,F3,F4,F5,F6,F7,F8,F9,F10,F11,F12,
    Delete, Home, End, PageUp, PageDown,
    Tab, CapsLock, LeftShift, RightShift, LeftAlt, RightAlt, LeftControl, RightControl, Windows, NumLock, Insert, PrintScreen, PauseBreak,*/
    NonBreakingSpace = 255,
}


static mut shifted: bool = false;

pub fn change_shift_state(scancode: u8) {
    let is_keydown: bool = scancode & 0x80 == 0;
    if is_keydown {
        match scancode {
            0x2A | 0x36 => unsafe { shifted = true },
            _ => (),
        }
    } else {
        let scancode_lower = scancode & !0x80u8;
        match scancode_lower {
            0x2A | 0x36 => unsafe { shifted = false },
            _ => (),
        }
    }
}

extern "x86-interrupt" fn keyboard_handler(stack_frame: &mut ExceptionStackFrame) {
    //println!("The Keyboard Was Pressed:\n{:#?}", stack_frame);
    let mut scan_code;
    unsafe {
        scan_code = inb(0x60);
        println!("One: {}",scan_code);
    }
    change_shift_state(scan_code);
    
    if scan_code == 224 {
        unsafe {
            scan_code = inb(0x60);
            println!("Two: {}",scan_code);
        }
    }
    
    unsafe {
        outb(0x20, 0x20);        
    }

    let character = match scan_code {
        /*1  => KeyCode::Escape,
        28 => KeyCode::Enter,
        57 => KeyCode::Space,
        59 => KeyCode::F1,
        60 => KeyCode::F2,
        61 => KeyCode::F3,
        62 => KeyCode::F4,
        63 => KeyCode::F5,
        64 => KeyCode::F6,
        65 => KeyCode::F7,
        66 => KeyCode::F8,
        67 => KeyCode::F9,
        68 => KeyCode::F10,
        87 => KeyCode::F11,
        88 => KeyCode::F12,*/
        2 | 79 => KeyCode::One,
        3 | 80 => KeyCode::Two,
        4 | 81 => KeyCode::Three,/*
        5 | 75 => KeyCode::Four,
        6 | 76 => KeyCode::Five,
        7 | 77 => KeyCode::Six,
        8 | 71 => KeyCode::Seven,
        9 | 72 => KeyCode::Eight,
        10 | 73 => KeyCode::Nine,
        11 | 82 => KeyCode::Zero,
        12 | 74 => KeyCode::Minus,
        13 => KeyCode::Equal,
        14 => KeyCode::BackSpace,
        15 => KeyCode::Tab,
        16 => KeyCode::q,
        17 => KeyCode::w,
        18 => KeyCode::e,
        19 => KeyCode::r,
        20 => KeyCode::t,
        21 => KeyCode::y,
        22 => KeyCode::u,
        23 => KeyCode::i,
        24 => KeyCode::o,
        25 => KeyCode::p,
        26 => KeyCode::LeftBracket,
        27 => KeyCode::RightBracket,
        30 => KeyCode::a,
        31 => KeyCode::s,
        32 => KeyCode::d,
        33 => KeyCode::f,
        34 => KeyCode::g,
        35 => KeyCode::h,
        36 => KeyCode::j,
        37 => KeyCode::k,
        38 => KeyCode::l,
        39 => KeyCode::Semicolon,
        40 => KeyCode::Apostrophe,
        41 => KeyCode::Grave,
        43 => KeyCode::BackSlash,
        44 => KeyCode::z,
        45 => KeyCode::x,
        46 => KeyCode::c,
        47 => KeyCode::v,
        48 => KeyCode::b,
        49 => KeyCode::n,
        50 => KeyCode::m,
        51 => KeyCode::Comma,
        52 => KeyCode::FullStop,
        53 => KeyCode::Slash,
        55 => KeyCode::Asterisk,
        78 => KeyCode::Plus,*/
        _ => KeyCode::NonBreakingSpace,
    };
    //SCREEN_WRITER.lock().write_byte(character as u8);
}

// ### Real Time Clock (RTC)

extern "x86-interrupt" fn rtc_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("RTC:\n{:#?}", stack_frame);
    unsafe {
        outb(0x20,0x20);
        outb(0xA0,0x20);
    }
    {}
}