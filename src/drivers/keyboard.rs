// # Keyboard Driver

// ## Prelude

use x86_64::instructions::interrupts;
use x86_64::instructions::port::{inb, outb};

// #### Code Page 437

/*
    Null = 0,//, LightSmile, DarkSmile, Heart, Diamond, CLub, Bullet, BulletBackground, Circle, CircleBackground, Mars, Venus, EighthNote, SixteenthNote, Sun,
    //RightTriangle, LeftTriangle, DoubleArrowVertical, DoubleExclamation, Pilcrow, Section, Bar, DoubleArrowBottom, UpArrow, DownArrow, RightArrow, LeftArrow, RightAngle, DoubleArrowHorz, UpTriangle, DownTriangle,
    Space = 32, Exclamation = 33, Quote = 34, Hash = 35, Dollar = 36, Percent = 37, Ampersand = 38, Apostrophe = 39, LeftParenthesis = 40, RightParenthesis = 41, Asterisk = 42, Plus = 43, Comma = 44, Minus = 45, FullStop = 46, Slash = 47,
    Zero = 48, One = 49, Two = 50, Three = 51, Four = 52, Five = 53, Six = 54, Seven = 55, Eight = 56, Nine = 57, Colon = 58, Semicolon = 59, LeftChevron = 60, Equal = 61, RightChevron = 62,  Question = 63,
    At = 64, A = 65, B = 66, C = 67, D = 68, E = 69, F = 70, G = 71, H = 72, I = 73, J = 74, K = 75, L = 76, M = 77, N = 78, O = 79, P = 80, Q = 81, R = 82, S = 83, T = 84, U = 85, V = 86, W = 87, X = 88, Y = 89, Z = 90, LeftBracket = 91, BackSlash = 92, RightBracket = 93, Caret = 94, Underscore = 95,
    Grave = 96, a = 97, b = 98, c = 99, d = 100, e = 101, f = 102, g = 103, h = 104, i = 105, j = 106, k = 107, l = 108, m = 109, n = 110, o = 111, p = 112, q = 113, r = 114, s = 115, t = 116, u = 117, v = 118, w = 119, x = 120, y = 121, z = 122, LeftBrace = 123, Pipe = 124, RightBrace = 125, Tilde = 126, House = 127,
    /*C_Cedilla, u_Umlaut, e_Acute,  a_Circumflex, a_Umlaut, a_Grave, a_Volle, c_Cedilla, e_Circumflex, e_Umlaut, e_Grave, i_Umlaut, i_Circumflex, i_Grave, A_Umlaut, A_Volle, 
    E_Acute, ae, AE, o_Circumflex, o_Umlaut, o_Grave, u_Circumflex, u_Grave, y_Umlaut, O_Umlaut, U_Umlaut, Cents, PoundSterling, Yen, Pesta, ScriptF,
    a_Acute, i_Acute, o_acute, u_acute, n_Tilde, N_Tilde, a_Ordinal, o_Ordinal, InvertedQuestion, LeftNegation, RightNegation, Half, Quarter, InvertedExclamation, LeftAngleQuotes, RightAngleQuotes, 
    LightBlock, MediumBlock, BoxDrawing179, BoxDrawing180, BoxDrawing181, BoxDrawing182, BoxDrawing183, BoxDrawing184, BoxDrawing185, BoxDrawing186, BoxDrawing187, BoxDrawing188, BoxDrawing189, BoxDrawing190, BoxDrawing191,
    BoxDrawing192, BoxDrawing193, BoxDrawing194, BoxDrawing195, BoxDrawing196, BoxDrawing197, BoxDrawing198, BoxDrawing199, BoxDrawing200, BoxDrawing201, BoxDrawing202, BoxDrawing203, BoxDrawing204, BoxDrawing205, BoxDrawing206, BoxDrawing207,
    BoxDrawing208, BoxDrawing209, BoxDrawing210, BoxDrawing211, BoxDrawing212, BoxDrawing213, BoxDrawing214, BoxDrawing215, BoxDrawing216, BoxDrawing217, BoxDrawing218, SolidBlock, BoxDrawing220, BoxDrawing221, BoxDrawing222, BoxDrawing223,
    alpha, beta, Gamma, pi, Sigma, sigma, mu, tau, Phi, Theta, omega, delta, Lemniscate, phi, epsilon, Intersection, TripleBar, PlusMinus, GreaterThanEqual, LessThanEqual, IntegrateTop, IntegrateBottom, Divide, Approximate, Degree, Bullet2, Interrupt, SquareRoot, NthPower, Squared, Square, */
*/

// #### Keyboard Key Mappings

// This table represents Scan Code Set 1 for keyboards.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyCode {
    // Keydown
    EscapeDown = 0x01,
    OneDown   = 0x02,
    TwoDown   = 0x03,
    ThreeDown = 0x04,
    FourDown  = 0x05,
    FiveDown = 0x06,
    SixDown  = 0x07,
    SevenDown = 0x08,
    EightDown = 0x09,
    NineDown = 0x0A,
    ZeroDown = 0x0B,
    MinusDown = 0x0C,
    EqualDown = 0x0D,
    BackspaceDown = 0x0E,
    TabDown = 0x0F,
    QDown = 0x10,
    WDown = 0x11,
    EDown = 0x12,
    RDown = 0x13,
    TDown = 0x14,
    YDown = 0x15,
    UDown = 0x16,
    IDown = 0x17,
    ODown = 0x18,
    PDown = 0x19,
    LeftBracketDown = 0x1A,
    RightBracketDown = 0x1B,
    EnterDown = 0x1C,
    LeftControlDown = 0x1D,
    ADown = 0x1E,
    SDown = 0x1F,
    DDown = 0x20,
    FDown = 0x21,
    GDown = 0x22,
    HDown = 0x23,
    JDown = 0x24,
    KDown = 0x25,
    LDown = 0x26,
    SemiColonDown = 0x27,
    ApostropheDown = 0x28,
    GraveDown = 0x29,
    LeftShiftDown = 0x2A,
    BackslashDown = 0x2B,
    ZDown = 0x2C,
    XDown = 0x2D,
    CDown = 0x2E,
    VDown = 0x2F,
    BDown = 0x30,
    NDown = 0x31,
    MDown = 0x32,
    CommaDown = 0x33,
    PeriodDown = 0x34,
    SlashDown = 0x35,
    RightShiftDown = 0x36,
    KeypadAsteriskDown = 0x37,
    LeftAltDown = 0x38,
    SpaceDown = 0x39,
    CapsLockDown = 0x3A,
    F1Down = 0x3B,
    F2Down = 0x3C,
    F3Down = 0x3D,
    F4Down = 0x3E,
    F5Down = 0x3F,
    F6Down = 0x40,
    F7Down = 0x41,
    F8Down = 0x42,
    F9Down = 0x43,
    F10Down = 0x44,
    NumberLockDown = 0x45,
    ScrollLockDown = 0x46,
    KeypadSevenDown = 0x47,
    KeypadEightDown = 0x48,
    KeypadNineDown = 0x49,
    KeypadMinusDown = 0x4A,
    KeypadFourDown = 0x4B,
    KeypadFiveDown = 0x4C,
    KeypadSixDown = 0x4D,
    KeypadPlusDown = 0x4E,
    KeypadOneDown = 0x4F,
    KeypadTwoDown = 0x50,
    KeypadThreeDown = 0x51,
    KeypadZeroDown = 0x52,
    KeypadPeriodDown = 0x53,
    F11Down = 0x57,
    F12Down = 0x58,
    // Keyup
}

/*pub enum KeyUpCode {
    Null,    
    Exclamation = 33, Quote = 34, Hash = 35, Dollar = 36, Percent = 37, Ampersand = 38, Apostrophe = 39, LeftParenthesis = 40, RightParenthesis = 41, Asterisk = 42, Plus = 43, Comma = 44, Minus = 45, FullStop = 46, Slash = 47,
    Zero = 48, One = 49, Two = 50, Three = 51, Four = 52, Five = 53, Six = 54, Seven = 55, Eight = 56, Nine = 57, Colon = 58, Semicolon = 59, LeftChevron = 60, Equal = 61, RightChevron = 62,  Question = 63,
    At = 64, A = 65, B = 66, C = 67, D = 68, E = 69, F = 70, G = 71, H = 72, I = 73, J = 74, K = 75, L = 76, M = 77, N = 78, O = 79, P = 80, Q = 81, R = 82, S = 83, T = 84, U = 85, V = 86, W = 87, X = 88, Y = 89, Z = 90, LeftBracket = 91, BackSlash = 92, RightBracket = 93, Caret = 94, Underscore = 95,
    Grave = 96, a = 97, b = 98, c = 99, d = 100, e = 101, f = 102, g = 103, h = 104, i = 105, j = 106, k = 107, l = 108, m = 109, n = 110, o = 111, p = 112, q = 113, r = 114, s = 115, t = 116, u = 117, v = 118, w = 119, x = 120, y = 121, z = 122, LeftBrace = 123, Pipe = 124, RightBrace = 125, Tilde = 126,
    Space = 255, 
    Enter = 256,
    Escape = 257,
    BackSpace = 258,
    F1 = 259, F2 = 260, F3 = 261, F4 = 262, F5 = 263, F6 = 264, F7 = 265, F8 = 266, F9 = 267, F10 = 268, F11 = 269, F12 = 270,
    Delete = 271, Home = 272, End = 273, PageUp = 274, PageDown = 275,
    Tab = 276, CapsLock = 277, LeftShift = 278, RightShift = 279, LeftAlt = 280, RightAlt = 281, LeftControl = 282, RightControl = 283, Windows = 284, NumLock = 285, Insert = 286, PrintScreen = 287, PauseBreak = 288,
}*/


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

fn keyboard_handler() {
    unsafe {
        interrupts::disable();
    }
    println!("Keyboard");
    //let mut scan_codes: [u8; 6]  = [255; 6];

    /*
    let mut x = 0xE1;
    x = x << 8 | 0x1D;
    x = x << 8 | 0x45;
    x = x << 8 | 0xE1;
    x = x << 8 | 0x9D;
    x = x << 8 | 0xC5;
    let PauseBreak = 0xE11D45E19DC5;
    println!("{:b}", x);
    println!("{}", PauseBreak == x);
    //println!("{:?}", x == y);*/
    /*

    println!("{} {:b}", scan_code, scan_code);
    print!("\n");
    */
    let scan_code = unsafe { inb(0x60) };
    let foo = scan_code as *const KeyCode;
    
    //println!("{:x}", KeyCode::ADown as u8);

    unsafe {
        outb(0x20, 0x20);        
    };

    unsafe {
        interrupts::enable();
    }
}

fn getScancode() -> u8 {
    let mut c: u8 = 0;
    loop {

        if(unsafe { inb(0x60) != c }) {
            unsafe {
                c = inb(0x60);
            };
            if(c > 0) {
                break;           
            }
        }
    }
    c
}


pub struct Keyboard {
    left_shift: bool,
    right_shift: bool,
    left_ctrl: bool,
    right_ctrl: bool,
    left_alt: bool,
    right_alt: bool,
    caps_lock: bool,
    num_lock:bool,
    insert: bool,
    character_buffer: [KeyCode; 128],    
}

impl Keyboard {

    pub fn new() -> Keyboard {
        Keyboard {
            left_shift: false,
            right_shift: false,
            left_ctrl: false,
            right_ctrl: false,
            left_alt: false,
            right_alt: false,
            caps_lock: false,
            num_lock:false,
            insert: false,
            character_buffer: [KeyCode::EscapeDown; 128],
        }
    }

    fn read_character() {
        //println!("The Keyboard Was Pressed:\n{:#?}", stack_frame);
        let mut scan_code;
        unsafe {
            scan_code = inb(0x60);
        };
        
       
        /*
        if scan_code == 224 {
            unsafe {
                scan_code = inb(0x60);
                //println!("Two: {}",scan_code);
            };
        };*/
        
        unsafe {
            outb(0x20, 0x20);        
        };
        /*
        let character = match scan_code {
            1  => KeyDownCode::Escape,
            28 => KeyDownCode::Enter,
            57 => KeyDownCode::Space,
            59 => KeyDownCode::F1,
            60 => KeyDownCode::F2,
            61 => KeyDownCode::F3,
            62 => KeyDownCode::F4,
            63 => KeyDownCode::F5,
            64 => KeyDownCode::F6,
            65 => KeyDownCode::F7,
            66 => KeyDownCode::F8,
            67 => KeyDownCode::F9,
            68 => KeyDownCode::F10,
            87 => KeyDownCode::F11,
            88 => KeyDownCode::F12,
            2 | 79 => KeyDownCode::One,
            3 | 80 => KeyDownCode::Two,
            4 | 81 => KeyDownCode::Three,
            5 | 75 => KeyDownCode::Four,
            6 | 76 => KeyDownCode::Five,
            7 | 77 => KeyDownCode::Six,
            8 | 71 => KeyDownCode::Seven,
            9 | 72 => KeyDownCode::Eight,
            10 | 73 => KeyDownCode::Nine,
            11 | 82 => KeyDownCode::Zero,
            12 | 74 => KeyDownCode::Minus,
            13 => KeyDownCode::Equal,
            14 => KeyDownCode::BackSpace,
            15 => KeyDownCode::Tab,
            16 => KeyDownCode::q,
            17 => KeyDownCode::w,
            18 => KeyDownCode::e,
            19 => KeyDownCode::r,
            20 => KeyDownCode::t,
            21 => KeyDownCode::y,
            22 => KeyDownCode::u,
            23 => KeyDownCode::i,
            24 => KeyDownCode::o,
            25 => KeyDownCode::p,
            26 => KeyDownCode::LeftBracket,
            27 => KeyDownCode::RightBracket,
            30 => KeyDownCode::a,
            31 => KeyDownCode::s,
            32 => KeyDownCode::d,
            33 => KeyDownCode::f,
            34 => KeyDownCode::g,
            35 => KeyDownCode::h,
            36 => KeyDownCode::j,
            37 => KeyDownCode::k,
            38 => KeyDownCode::l,
            39 => KeyDownCode::Semicolon,
            40 => KeyDownCode::Apostrophe,
            41 => KeyDownCode::Grave,
            43 => KeyDownCode::BackSlash,
            44 => KeyDownCode::z,
            45 => KeyDownCode::x,
            46 => KeyDownCode::c,
            47 => KeyDownCode::v,
            48 => KeyDownCode::b,
            49 => KeyDownCode::n,
            50 => KeyDownCode::m,
            51 => KeyDownCode::Comma,
            52 | 83 => KeyDownCode::FullStop,
            53 => KeyDownCode::Slash,
            55 => KeyDownCode::Asterisk,
            78 => KeyDownCode::Plus,
            _ => KeyDownCode::Null,
        };
        character*/
    }
}
