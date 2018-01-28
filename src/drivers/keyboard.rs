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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyCode {
    Null, Escape, One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero, Minus, Equal, Backspace,
    Tab, Q, W, E, R, T, Y, U, I, O, P, LeftBracket, RightBracket, Enter, 
    LeftControl, A, S, D, F, G, H, J, K, L, SemiColon, Apostrophe,
    Grave, LeftShift, Backslash, Z, X, C, V, B, N, M, Comma, Period, Slash, RightShift, KeypadAsterisk,
    LeftAlt, Space, CapsLock, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10,
    NumberLock, ScrollLock, KeypadSeven, KeypadEight, KeypadNine, KeypadMinus, KeypadFour, KeypadFive, KeypadSix, KeypadPlus, KeypadOne, KeypadTwo, KeypadThree, KeypadZero, KeypadPeriod,
    F11, F12, PreviousTrack, NextTrack, KeypadEnter, RightControl, Mute, Calculator, Play, Stop, VolumeDown, VolumeUp, WWWHome, KeypadSlash,
    RightAlt, Home, CursorUp, PageUp, CursorLeft, CursorRight, End, CursorDown, PageDown, Insert, Delete, LeftGUI, RightGUI, Apps, Power, Sleep, Wake,
    WWWSearch, WWWFavorites, WWWRefresh, WWWStop, WWWForward, WWWBack,
    MyComputer, Email, MediaSelect, PrintScreen, PauseBreak,
}



// This table represents Scan Code Set 1 for keyboards.

pub fn scan_to_key(scan_code: u32) -> Option<(KeyCode, KeyState)> {
    let key_state: (KeyCode, KeyState) = match scan_code {
        // Key Down
        0x01 => (KeyCode::Escape, KeyState::Down),
        0x02 => (KeyCode::One, KeyState::Down),
        0x03 => (KeyCode::Two, KeyState::Down),
        0x04 => (KeyCode::Three, KeyState::Down),
        0x05 => (KeyCode::Four, KeyState::Down),
        0x06 => (KeyCode::Five, KeyState::Down),
        0x07 => (KeyCode::Six, KeyState::Down),
        0x08 => (KeyCode::Seven, KeyState::Down),
        0x09 => (KeyCode::Eight, KeyState::Down),
        0x0A => (KeyCode::Nine, KeyState::Down),
        0x0B => (KeyCode::Zero, KeyState::Down),
        0x0C => (KeyCode::Minus, KeyState::Down),
        0x0D => (KeyCode::Equal, KeyState::Down),
        0x0E => (KeyCode::Backspace, KeyState::Down),
        0x0F => (KeyCode::Tab, KeyState::Down),
        0x10 => (KeyCode::Q, KeyState::Down),
        0x11 => (KeyCode::W, KeyState::Down),
        0x12 => (KeyCode::E, KeyState::Down),
        0x13 => (KeyCode::R, KeyState::Down),
        0x14 => (KeyCode::T, KeyState::Down),
        0x15 => (KeyCode::Y, KeyState::Down),
        0x16 => (KeyCode::U, KeyState::Down),
        0x17 => (KeyCode::I, KeyState::Down),
        0x18 => (KeyCode::O, KeyState::Down),
        0x19 => (KeyCode::P, KeyState::Down),
        0x1A => (KeyCode::LeftBracket, KeyState::Down),
        0x1B => (KeyCode::RightBracket, KeyState::Down),
        0x1C => (KeyCode::Enter, KeyState::Down),
        0x1D => (KeyCode::LeftControl, KeyState::Down),
        0x1E => (KeyCode::A, KeyState::Down),
        0x1F => (KeyCode::S, KeyState::Down),
        0x20 => (KeyCode::D, KeyState::Down),
        0x21 => (KeyCode::F, KeyState::Down),
        0x22 => (KeyCode::G, KeyState::Down),
        0x23 => (KeyCode::H, KeyState::Down),
        0x24 => (KeyCode::J, KeyState::Down),
        0x25 => (KeyCode::K, KeyState::Down),
        0x26 => (KeyCode::L, KeyState::Down),
        0x27 => (KeyCode::SemiColon, KeyState::Down),
        0x28 => (KeyCode::Apostrophe, KeyState::Down),
        0x29 => (KeyCode::Grave, KeyState::Down),
        0x2A => (KeyCode::LeftShift, KeyState::Down),
        0x2B => (KeyCode::Backslash, KeyState::Down),
        0x2C => (KeyCode::Z, KeyState::Down),
        0x2D => (KeyCode::X, KeyState::Down),
        0x2E => (KeyCode::C, KeyState::Down),
        0x2F => (KeyCode::V, KeyState::Down),
        0x30 => (KeyCode::B, KeyState::Down),
        0x31 => (KeyCode::N, KeyState::Down),
        0x32 => (KeyCode::M, KeyState::Down),
        0x33 => (KeyCode::Comma, KeyState::Down),
        0x34 => (KeyCode::Period, KeyState::Down),
        0x35 => (KeyCode::Slash, KeyState::Down),
        0x36 => (KeyCode::RightShift, KeyState::Down),
        0x37 => (KeyCode::KeypadAsterisk, KeyState::Down),
        0x38 => (KeyCode::LeftAlt, KeyState::Down),
        0x39 => (KeyCode::Space, KeyState::Down),
        0x3A => (KeyCode::CapsLock, KeyState::Down),
        0x3B => (KeyCode::F1, KeyState::Down),
        0x3C => (KeyCode::F2, KeyState::Down),
        0x3D => (KeyCode::F3, KeyState::Down),
        0x3E => (KeyCode::F4, KeyState::Down),
        0x3F => (KeyCode::F5, KeyState::Down),
        0x40 => (KeyCode::F6, KeyState::Down),
        0x41 => (KeyCode::F7, KeyState::Down),
        0x42 => (KeyCode::F8, KeyState::Down),
        0x43 => (KeyCode::F9, KeyState::Down),
        0x44 => (KeyCode::F10, KeyState::Down),
        0x45 => (KeyCode::NumberLock, KeyState::Down),
        0x46 => (KeyCode::ScrollLock, KeyState::Down),
        0x47 => (KeyCode::KeypadSeven, KeyState::Down),
        0x48 => (KeyCode::KeypadEight, KeyState::Down),
        0x49 => (KeyCode::KeypadNine, KeyState::Down),
        0x4A => (KeyCode::KeypadMinus, KeyState::Down),
        0x4B => (KeyCode::KeypadFour, KeyState::Down),
        0x4C => (KeyCode::KeypadFive, KeyState::Down),
        0x4D => (KeyCode::KeypadSix, KeyState::Down),
        0x4E => (KeyCode::KeypadPlus, KeyState::Down),
        0x4F => (KeyCode::KeypadOne, KeyState::Down),
        0x50 => (KeyCode::KeypadTwo, KeyState::Down),
        0x51 => (KeyCode::KeypadThree, KeyState::Down),
        0x52 => (KeyCode::KeypadZero, KeyState::Down),
        0x53 => (KeyCode::KeypadPeriod, KeyState::Down),
        0x57 => (KeyCode::F11, KeyState::Down),
        0x58 => (KeyCode::F12, KeyState::Down),
        0xE01C => (KeyCode::KeypadEnter, KeyState::Down),
        0xE01D => (KeyCode::RightControl, KeyState::Down),
        0xE020 => (KeyCode::Mute, KeyState::Down),
        0xE021 => (KeyCode::Calculator, KeyState::Down),
        0xE022 => (KeyCode::Play, KeyState::Down),
        0xE024 => (KeyCode::Stop, KeyState::Down),
        0xE02E => (KeyCode::VolumeDown, KeyState::Down),
        0xE030 => (KeyCode::VolumeUp, KeyState::Down),
        0xE032 => (KeyCode::WWWHome, KeyState::Down),
        0xE035 => (KeyCode::KeypadSlash, KeyState::Down),
        0xE038 => (KeyCode::RightAlt, KeyState::Down),
        0xE047 => (KeyCode::Home, KeyState::Down),
        0xE048 => (KeyCode::CursorUp, KeyState::Down),
        0xE049 => (KeyCode::PageUp, KeyState::Down),
        0xE04B => (KeyCode::CursorLeft, KeyState::Down),
        0xE04D => (KeyCode::CursorRight, KeyState::Down),
        0xE04F => (KeyCode::End, KeyState::Down),
        0xE050 => (KeyCode::CursorDown, KeyState::Down),
        0xE051 => (KeyCode::PageDown, KeyState::Down),
        0xE052 => (KeyCode::Insert, KeyState::Down),
        0xE053 => (KeyCode::Delete, KeyState::Down),
        0xE05B => (KeyCode::LeftGUI, KeyState::Down),
        0xE05C => (KeyCode::RightGUI, KeyState::Down),
        0xE05D => (KeyCode::Apps, KeyState::Down),
        0xE05E => (KeyCode::Power, KeyState::Down),
        0xE05F => (KeyCode::Sleep, KeyState::Down),
        0xE063 => (KeyCode::Wake, KeyState::Down),
        0xE065 => (KeyCode::WWWSearch, KeyState::Down),
        0xE066 => (KeyCode::WWWFavorites, KeyState::Down),
        0xE067 => (KeyCode::WWWRefresh, KeyState::Down),
        0xE068 => (KeyCode::WWWStop, KeyState::Down),
        0xE069 => (KeyCode::WWWForward, KeyState::Down),
        0xE06A => (KeyCode::WWWBack, KeyState::Down),
        0xE06B => (KeyCode::MyComputer, KeyState::Down),
        0xE06C => (KeyCode::Email, KeyState::Down),
        0xE06D => (KeyCode::MediaSelect, KeyState::Down),
        0xE02AE037 => (KeyCode::PrintScreen, KeyState::Down),
        0xE11D45E19DC5 => (KeyCode::PauseBreak, KeyState::Down),

        // Key Up
        0x81 => (KeyCode::Escape, KeyState::Up),
        0x82 => (KeyCode::One, KeyState::Up),
        0x83 => (KeyCode::Two, KeyState::Up),
        0x84 => (KeyCode::Three, KeyState::Up),
        0x85 => (KeyCode::Four, KeyState::Up),
        0x86 => (KeyCode::Five, KeyState::Up),
        0x87 => (KeyCode::Six, KeyState::Up),
        0x88 => (KeyCode::Seven, KeyState::Up),
        0x89 => (KeyCode::Eight, KeyState::Up),
        0x8A => (KeyCode::Nine, KeyState::Up),
        0x8B => (KeyCode::Zero, KeyState::Up),
        0x8C => (KeyCode::Minus, KeyState::Up),
        0x8D => (KeyCode::Equal, KeyState::Up),
        0x8E => (KeyCode::Backspace, KeyState::Up),
        0x8F => (KeyCode::Tab, KeyState::Up),
        0x90 => (KeyCode::Q, KeyState::Up),
        0x91 => (KeyCode::W, KeyState::Up),
        0x92 => (KeyCode::E, KeyState::Up),
        0x93 => (KeyCode::R, KeyState::Up),
        0x94 => (KeyCode::T, KeyState::Up),
        0x95 => (KeyCode::Y, KeyState::Up),
        0x96 => (KeyCode::U, KeyState::Up),
        0x97 => (KeyCode::I, KeyState::Up),
        0x98 => (KeyCode::O, KeyState::Up),
        0x99 => (KeyCode::P, KeyState::Up),
        0x9A => (KeyCode::LeftBracket, KeyState::Up),
        0x9B => (KeyCode::RightBracket, KeyState::Up),
        0x9C => (KeyCode::Enter, KeyState::Up),
        0x9D => (KeyCode::LeftControl, KeyState::Up),
        0x9E => (KeyCode::A, KeyState::Up),
        0x9F => (KeyCode::S, KeyState::Up),
        0xA0 => (KeyCode::D, KeyState::Up),
        0xA1 => (KeyCode::F, KeyState::Up),
        0xA2 => (KeyCode::G, KeyState::Up),
        0xA3 => (KeyCode::H, KeyState::Up),
        0xA4 => (KeyCode::J, KeyState::Up),
        0xA5 => (KeyCode::K, KeyState::Up),
        0xA6 => (KeyCode::L, KeyState::Up),
        0xA7 => (KeyCode::SemiColon, KeyState::Up),
        0xA8 => (KeyCode::Apostrophe, KeyState::Up),
        0xA9 => (KeyCode::Grave, KeyState::Up),
        0xAA => (KeyCode::LeftShift, KeyState::Up),
        0xAB => (KeyCode::Backslash, KeyState::Up),
        0xAC => (KeyCode::Z, KeyState::Up),
        0xAD => (KeyCode::X, KeyState::Up),
        0xAE => (KeyCode::C, KeyState::Up),
        0xAF => (KeyCode::V, KeyState::Up),
        0xB0 => (KeyCode::B, KeyState::Up),
        0xB1 => (KeyCode::N, KeyState::Up),
        0xB2 => (KeyCode::M, KeyState::Up),
        0xB3 => (KeyCode::Comma, KeyState::Up),
        0xB4 => (KeyCode::Period, KeyState::Up),
        0xB5 => (KeyCode::Slash, KeyState::Up),
        0xB6 => (KeyCode::RightShift, KeyState::Up),
        0xB7 => (KeyCode::KeypadAsterisk, KeyState::Up),
        0xB8 => (KeyCode::LeftAlt, KeyState::Up),
        0xB9 => (KeyCode::Space, KeyState::Up),
        0xBA => (KeyCode::CapsLock, KeyState::Up),
        0xBB => (KeyCode::F1, KeyState::Up),
        0xBC => (KeyCode::F2, KeyState::Up),
        0xBD => (KeyCode::F3, KeyState::Up),
        0xBE => (KeyCode::F4, KeyState::Up),
        0xBF => (KeyCode::F5, KeyState::Up),
        0xC0 => (KeyCode::F6, KeyState::Up),
        0xC1 => (KeyCode::F7, KeyState::Up),
        0xC2 => (KeyCode::F8, KeyState::Up),
        0xC3 => (KeyCode::F9, KeyState::Up),
        0xC4 => (KeyCode::F10, KeyState::Up),
        0xC5 => (KeyCode::NumberLock, KeyState::Up),
        0xC6 => (KeyCode::ScrollLock, KeyState::Up),
        0xC7 => (KeyCode::KeypadSeven, KeyState::Up),
        0xC8 => (KeyCode::KeypadEight, KeyState::Up),
        0xC9 => (KeyCode::KeypadNine, KeyState::Up),
        0xCA => (KeyCode::KeypadMinus, KeyState::Up),
        0xCB => (KeyCode::KeypadFour, KeyState::Up),
        0xCC => (KeyCode::KeypadFive, KeyState::Up),
        0xCD => (KeyCode::KeypadSix, KeyState::Up),
        0xCE => (KeyCode::KeypadPlus, KeyState::Up),
        0xCF => (KeyCode::KeypadOne, KeyState::Up),
        0xD0 => (KeyCode::KeypadTwo, KeyState::Up),
        0xD1 => (KeyCode::KeypadThree, KeyState::Up),
        0xD2 => (KeyCode::KeypadZero, KeyState::Up),
        0xD3 => (KeyCode::KeypadPeriod, KeyState::Up),
        0xD7 => (KeyCode::F11, KeyState::Up),
        0xD8 => (KeyCode::F12, KeyState::Up),
        0xE090 => (KeyCode::PreviousTrack, KeyState::Up),
        0xE099 => (KeyCode::NextTrack, KeyState::Up),
        0xE09C => (KeyCode::KeypadEnter, KeyState::Up),
        0xE09D => (KeyCode::RightControl, KeyState::Up),
        0xE0A0 => (KeyCode::Mute, KeyState::Up),
        0xE0A1 => (KeyCode::Calculator, KeyState::Up),
        0xE0A2 => (KeyCode::Play, KeyState::Up),
        0xE0A4 => (KeyCode::Stop, KeyState::Up),
        0xE0AE => (KeyCode::VolumeDown, KeyState::Up),
        0xE0B0 => (KeyCode::VolumeUp, KeyState::Up),
        0xE0B2 => (KeyCode::WWWHome, KeyState::Up),
        0xE0B5 => (KeyCode::KeypadSlash, KeyState::Up),
        0xE0B8 => (KeyCode::RightAlt, KeyState::Up),
        0xE0C7 => (KeyCode::Home, KeyState::Up),
        0xE0C8 => (KeyCode::CursorUp, KeyState::Up),
        0xE0C9 => (KeyCode::PageUp, KeyState::Up),
        0xE0CB => (KeyCode::CursorLeft, KeyState::Up),
        0xE0CD => (KeyCode::CursorRight, KeyState::Up),
        0xE0CF => (KeyCode::End, KeyState::Up),
        0xE0D0 => (KeyCode::CursorDown, KeyState::Up),
        0xE0D1 => (KeyCode::PageDown, KeyState::Up),
        0xE0D2 => (KeyCode::Insert, KeyState::Up),
        0xE0D3 => (KeyCode::Delete, KeyState::Up),
        0xE0DB => (KeyCode::LeftGUI, KeyState::Up),
        0xE0DC => (KeyCode::RightGUI, KeyState::Up),
        0xE0DD => (KeyCode::Apps, KeyState::Up),
        0xE0DE => (KeyCode::Power, KeyState::Up),
        0xE0DF => (KeyCode::Sleep, KeyState::Up),
        0xE0E3 => (KeyCode::Wake, KeyState::Up),
        0xE0E5 => (KeyCode::WWWSearch, KeyState::Up),
        0xE0E6 => (KeyCode::WWWFavorites, KeyState::Up),
        0xE0E7 => (KeyCode::WWWRefresh, KeyState::Up),
        0xE0E8 => (KeyCode::WWWStop, KeyState::Up),
        0xE0E9 => (KeyCode::WWWForward, KeyState::Up),
        0xE0EA => (KeyCode::WWWBack, KeyState::Up),
        0xE0EB => (KeyCode::MyComputer, KeyState::Up),
        0xE0EC => (KeyCode::Email, KeyState::Up),
        0xE0ED => (KeyCode::MediaSelect, KeyState::Up),
        0xE0B7E0AA => (KeyCode::PrintScreen, KeyState::Up),
        _ => (KeyCode::Null, KeyState::Null),
    };

    match key_state {
        (KeyCode::Null, _) => None,
        _ => Some(key_state),
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyState {
    Null,
    Down,
    Up,
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


static mut current_byte: u32 = 0;

pub unsafe fn read_byte() {

    let scan_code = inb(0x60);
    let full_code = scan_code as u32 | current_byte;
    let key_code = scan_to_key(full_code);

    match key_code {
        Some((code, state)) => {
            current_byte = 0;
            if state == KeyState::Down {
                print!("{:?}", code)
            }
            
        },
        None => {
            current_byte = full_code << 8;
        },
    };
}
/*
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
*/