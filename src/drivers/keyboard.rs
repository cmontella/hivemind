// # Keyboard Driver

// ## Prelude

use x86_64::instructions::interrupts;
use x86_64::instructions::port::{inb, outb};
use spin::Mutex;
use alloc;
use drivers::vga::{SCREEN_WRITER, ColorCode, Color};
use interrupts::event;
use mech::database::{Transaction, Change};
use mech::table::{Table, Value};
use alloc::String;
use ::MechDB;

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
        // Key Down                                         Key Up
        0x01=> (KeyCode::Escape, KeyState::Down),	        0x81 => (KeyCode::Escape, KeyState::Up),
        0x02 => (KeyCode::One, KeyState::Down),	            0x82 => (KeyCode::One, KeyState::Up),
        0x03 => (KeyCode::Two, KeyState::Down),	            0x83 => (KeyCode::Two, KeyState::Up),
        0x04 => (KeyCode::Three, KeyState::Down),   	    0x84 => (KeyCode::Three, KeyState::Up),
        0x05 => (KeyCode::Four, KeyState::Down),	        0x85 => (KeyCode::Four, KeyState::Up),
        0x06 => (KeyCode::Five, KeyState::Down),    	    0x86 => (KeyCode::Five, KeyState::Up),
        0x07 => (KeyCode::Six, KeyState::Down),	            0x87 => (KeyCode::Six, KeyState::Up),
        0x08 => (KeyCode::Seven, KeyState::Down),   	    0x88 => (KeyCode::Seven, KeyState::Up),
        0x09 => (KeyCode::Eight, KeyState::Down),	        0x89 => (KeyCode::Eight, KeyState::Up),
        0x0A => (KeyCode::Nine, KeyState::Down),	        0x8A => (KeyCode::Nine, KeyState::Up),
        0x0B => (KeyCode::Zero, KeyState::Down),	        0x8B => (KeyCode::Zero, KeyState::Up),
        0x0C => (KeyCode::Minus, KeyState::Down),	        0x8C => (KeyCode::Minus, KeyState::Up),
        0x0D => (KeyCode::Equal, KeyState::Down),	        0x8D => (KeyCode::Equal, KeyState::Up),
        0x0E => (KeyCode::Backspace, KeyState::Down),   	0x8E => (KeyCode::Backspace, KeyState::Up),
        0x0F => (KeyCode::Tab, KeyState::Down),	            0x8F => (KeyCode::Tab, KeyState::Up),
        0x10 => (KeyCode::Q, KeyState::Down),	            0x90 => (KeyCode::Q, KeyState::Up),
        0x11 => (KeyCode::W, KeyState::Down),	            0x91 => (KeyCode::W, KeyState::Up),
        0x12 => (KeyCode::E, KeyState::Down),	            0x92 => (KeyCode::E, KeyState::Up),
        0x13 => (KeyCode::R, KeyState::Down),	            0x93 => (KeyCode::R, KeyState::Up),
        0x14 => (KeyCode::T, KeyState::Down),	            0x94 => (KeyCode::T, KeyState::Up),
        0x15 => (KeyCode::Y, KeyState::Down),            	0x95 => (KeyCode::Y, KeyState::Up),
        0x16 => (KeyCode::U, KeyState::Down),           	0x96 => (KeyCode::U, KeyState::Up),
        0x17 => (KeyCode::I, KeyState::Down),               0x97 => (KeyCode::I, KeyState::Up),
        0x18 => (KeyCode::O, KeyState::Down),           	0x98 => (KeyCode::O, KeyState::Up),
        0x19 => (KeyCode::P, KeyState::Down),           	0x99 => (KeyCode::P, KeyState::Up),
        0x1A => (KeyCode::LeftBracket, KeyState::Down),	    0x9A => (KeyCode::LeftBracket, KeyState::Up),
        0x1B => (KeyCode::RightBracket, KeyState::Down),	0x9B => (KeyCode::RightBracket, KeyState::Up),
        0x1C => (KeyCode::Enter, KeyState::Down),	        0x9C => (KeyCode::Enter, KeyState::Up),
        0x1D => (KeyCode::LeftControl, KeyState::Down),	    0x9D => (KeyCode::LeftControl, KeyState::Up),
        0x1E => (KeyCode::A, KeyState::Down),	            0x9E => (KeyCode::A, KeyState::Up),
        0x1F => (KeyCode::S, KeyState::Down),	            0x9F => (KeyCode::S, KeyState::Up),
        0x20 => (KeyCode::D, KeyState::Down),       	    0xA0 => (KeyCode::D, KeyState::Up),
        0x21 => (KeyCode::F, KeyState::Down),           	0xA1 => (KeyCode::F, KeyState::Up),
        0x22 => (KeyCode::G, KeyState::Down),           	0xA2 => (KeyCode::G, KeyState::Up),
        0x23 => (KeyCode::H, KeyState::Down),           	0xA3 => (KeyCode::H, KeyState::Up),
        0x24 => (KeyCode::J, KeyState::Down),           	0xA4 => (KeyCode::J, KeyState::Up),
        0x25 => (KeyCode::K, KeyState::Down),           	0xA5 => (KeyCode::K, KeyState::Up),
        0x26 => (KeyCode::L, KeyState::Down),           	0xA6 => (KeyCode::L, KeyState::Up),
        0x27 => (KeyCode::SemiColon, KeyState::Down),   	0xA7 => (KeyCode::SemiColon, KeyState::Up),
        0x28 => (KeyCode::Apostrophe, KeyState::Down),  	0xA8 => (KeyCode::Apostrophe, KeyState::Up),
        0x29 => (KeyCode::Grave, KeyState::Down),       	0xA9 => (KeyCode::Grave, KeyState::Up),
        0x2A => (KeyCode::LeftShift, KeyState::Down),   	0xAA => (KeyCode::LeftShift, KeyState::Up),
        0x2B => (KeyCode::Backslash, KeyState::Down),	    0xAB => (KeyCode::Backslash, KeyState::Up),
        0x2C => (KeyCode::Z, KeyState::Down),           	0xAC => (KeyCode::Z, KeyState::Up),
        0x2D => (KeyCode::X, KeyState::Down),           	0xAD => (KeyCode::X, KeyState::Up),
        0x2E => (KeyCode::C, KeyState::Down),           	0xAE => (KeyCode::C, KeyState::Up),
        0x2F => (KeyCode::V, KeyState::Down),           	0xAF => (KeyCode::V, KeyState::Up),
        0x30 => (KeyCode::B, KeyState::Down),              	0xB0 => (KeyCode::B, KeyState::Up),
        0x31 => (KeyCode::N, KeyState::Down),              	0xB1 => (KeyCode::N, KeyState::Up),
        0x32 => (KeyCode::M, KeyState::Down),              	0xB2 => (KeyCode::M, KeyState::Up),
        0x33 => (KeyCode::Comma, KeyState::Down),       	0xB3 => (KeyCode::Comma, KeyState::Up),
        0x34 => (KeyCode::Period, KeyState::Down),      	0xB4 => (KeyCode::Period, KeyState::Up),
        0x35 => (KeyCode::Slash, KeyState::Down),	        0xB5 => (KeyCode::Slash, KeyState::Up),
        0x36 => (KeyCode::RightShift, KeyState::Down),  	0xB6 => (KeyCode::RightShift, KeyState::Up),
        0x37 => (KeyCode::KeypadAsterisk, KeyState::Down),	0xB7 => (KeyCode::KeypadAsterisk, KeyState::Up),
        0x38 => (KeyCode::LeftAlt, KeyState::Down),     	0xB8 => (KeyCode::LeftAlt, KeyState::Up),
        0x39 => (KeyCode::Space, KeyState::Down),	        0xB9 => (KeyCode::Space, KeyState::Up),
        0x3A => (KeyCode::CapsLock, KeyState::Down),	    0xBA => (KeyCode::CapsLock, KeyState::Up),
        0x3B => (KeyCode::F1, KeyState::Down),	            0xBB => (KeyCode::F1, KeyState::Up),
        0x3C => (KeyCode::F2, KeyState::Down),  	        0xBC => (KeyCode::F2, KeyState::Up),
        0x3D => (KeyCode::F3, KeyState::Down),  	        0xBD => (KeyCode::F3, KeyState::Up),
        0x3E => (KeyCode::F4, KeyState::Down),  	        0xBE => (KeyCode::F4, KeyState::Up),
        0x3F => (KeyCode::F5, KeyState::Down),          	0xBF => (KeyCode::F5, KeyState::Up),
        0x40 => (KeyCode::F6, KeyState::Down),          	0xC0 => (KeyCode::F6, KeyState::Up),
        0x41 => (KeyCode::F7, KeyState::Down),          	0xC1 => (KeyCode::F7, KeyState::Up),
        0x42 => (KeyCode::F8, KeyState::Down),          	0xC2 => (KeyCode::F8, KeyState::Up),
        0x43 => (KeyCode::F9, KeyState::Down),          	0xC3 => (KeyCode::F9, KeyState::Up),
        0x44 => (KeyCode::F10, KeyState::Down),	            0xC4 => (KeyCode::F10, KeyState::Up),
        0x45 => (KeyCode::NumberLock, KeyState::Down),  	0xC5 => (KeyCode::NumberLock, KeyState::Up),
        0x46 => (KeyCode::ScrollLock, KeyState::Down),  	0xC6 => (KeyCode::ScrollLock, KeyState::Up),
        0x47 => (KeyCode::KeypadSeven, KeyState::Down), 	0xC7 => (KeyCode::KeypadSeven, KeyState::Up),
        0x48 => (KeyCode::KeypadEight, KeyState::Down),	    0xC8 => (KeyCode::KeypadEight, KeyState::Up),
        0x49 => (KeyCode::KeypadNine, KeyState::Down),  	0xC9 => (KeyCode::KeypadNine, KeyState::Up),
        0x4A => (KeyCode::KeypadMinus, KeyState::Down), 	0xCA => (KeyCode::KeypadMinus, KeyState::Up),
        0x4B => (KeyCode::KeypadFour, KeyState::Down),  	0xCB => (KeyCode::KeypadFour, KeyState::Up),
        0x4C => (KeyCode::KeypadFive, KeyState::Down),  	0xCC => (KeyCode::KeypadFive, KeyState::Up),
        0x4D => (KeyCode::KeypadSix, KeyState::Down),	    0xCD => (KeyCode::KeypadSix, KeyState::Up),
        0x4E => (KeyCode::KeypadPlus, KeyState::Down),  	0xCE => (KeyCode::KeypadPlus, KeyState::Up),
        0x4F => (KeyCode::KeypadOne, KeyState::Down),	    0xCF => (KeyCode::KeypadOne, KeyState::Up),
        0x50 => (KeyCode::KeypadTwo, KeyState::Down),	    0xD0 => (KeyCode::KeypadTwo, KeyState::Up),
        0x51 => (KeyCode::KeypadThree, KeyState::Down), 	0xD1 => (KeyCode::KeypadThree, KeyState::Up),
        0x52 => (KeyCode::KeypadZero, KeyState::Down),	    0xD2 => (KeyCode::KeypadZero, KeyState::Up),
        0x53 => (KeyCode::KeypadPeriod, KeyState::Down),	0xD3 => (KeyCode::KeypadPeriod, KeyState::Up),
        0x57 => (KeyCode::F11, KeyState::Down),	            0xD7 => (KeyCode::F11, KeyState::Up),
        0x58 => (KeyCode::F12, KeyState::Down),	            0xD8 => (KeyCode::F12, KeyState::Up),
        0xE010 => (KeyCode::PreviousTrack, KeyState::Down),	0xE090 => (KeyCode::PreviousTrack, KeyState::Up),
        0xE019 => (KeyCode::NextTrack, KeyState::Down),	    0xE099 => (KeyCode::NextTrack, KeyState::Up),
        0xE01C => (KeyCode::KeypadEnter, KeyState::Down),	0xE09C => (KeyCode::KeypadEnter, KeyState::Up),
        0xE01D => (KeyCode::RightControl, KeyState::Down),	0xE09D => (KeyCode::RightControl, KeyState::Up),
        0xE020 => (KeyCode::Mute, KeyState::Down),	        0xE0A0 => (KeyCode::Mute, KeyState::Up),
        0xE021 => (KeyCode::Calculator, KeyState::Down),	0xE0A1 => (KeyCode::Calculator, KeyState::Up),
        0xE022 => (KeyCode::Play, KeyState::Down),      	0xE0A2 => (KeyCode::Play, KeyState::Up),
        0xE024 => (KeyCode::Stop, KeyState::Down),      	0xE0A4 => (KeyCode::Stop, KeyState::Up),
        0xE02E => (KeyCode::VolumeDown, KeyState::Down),	0xE0AE => (KeyCode::VolumeDown, KeyState::Up),
        0xE030 => (KeyCode::VolumeUp, KeyState::Down),  	0xE0B0 => (KeyCode::VolumeUp, KeyState::Up),
        0xE032 => (KeyCode::WWWHome, KeyState::Down),	    0xE0B2 => (KeyCode::WWWHome, KeyState::Up),
        0xE035 => (KeyCode::KeypadSlash, KeyState::Down),	0xE0B5 => (KeyCode::KeypadSlash, KeyState::Up),
        0xE038 => (KeyCode::RightAlt, KeyState::Down),  	0xE0B8 => (KeyCode::RightAlt, KeyState::Up),
        0xE047 => (KeyCode::Home, KeyState::Down),      	0xE0C7 => (KeyCode::Home, KeyState::Up),
        0xE048 => (KeyCode::CursorUp, KeyState::Down),     	0xE0C8 => (KeyCode::CursorUp, KeyState::Up),
        0xE049 => (KeyCode::PageUp, KeyState::Down),    	0xE0C9 => (KeyCode::PageUp, KeyState::Up),
        0xE04B => (KeyCode::CursorLeft, KeyState::Down),	0xE0CB => (KeyCode::CursorLeft, KeyState::Up),
        0xE04D => (KeyCode::CursorRight, KeyState::Down),	0xE0CD => (KeyCode::CursorRight, KeyState::Up),
        0xE04F => (KeyCode::End, KeyState::Down),	        0xE0CF => (KeyCode::End, KeyState::Up),
        0xE050 => (KeyCode::CursorDown, KeyState::Down),	0xE0D0 => (KeyCode::CursorDown, KeyState::Up),
        0xE051 => (KeyCode::PageDown, KeyState::Down),  	0xE0D1 => (KeyCode::PageDown, KeyState::Up),
        0xE052 => (KeyCode::Insert, KeyState::Down),    	0xE0D2 => (KeyCode::Insert, KeyState::Up),
        0xE053 => (KeyCode::Delete, KeyState::Down),    	0xE0D3 => (KeyCode::Delete, KeyState::Up),
        0xE05B => (KeyCode::LeftGUI, KeyState::Down),   	0xE0DB => (KeyCode::LeftGUI, KeyState::Up),
        0xE05C => (KeyCode::RightGUI, KeyState::Down),  	0xE0DC => (KeyCode::RightGUI, KeyState::Up),
        0xE05D => (KeyCode::Apps, KeyState::Down),      	0xE0DD => (KeyCode::Apps, KeyState::Up),
        0xE05E => (KeyCode::Power, KeyState::Down),     	0xE0DE => (KeyCode::Power, KeyState::Up),
        0xE05F => (KeyCode::Sleep, KeyState::Down),     	0xE0DF => (KeyCode::Sleep, KeyState::Up),
        0xE063 => (KeyCode::Wake, KeyState::Down),      	0xE0E3 => (KeyCode::Wake, KeyState::Up),
        0xE065 => (KeyCode::WWWSearch, KeyState::Down), 	0xE0E5 => (KeyCode::WWWSearch, KeyState::Up),
        0xE066 => (KeyCode::WWWFavorites, KeyState::Down),	0xE0E6 => (KeyCode::WWWFavorites, KeyState::Up),
        0xE067 => (KeyCode::WWWRefresh, KeyState::Down),	0xE0E7 => (KeyCode::WWWRefresh, KeyState::Up),
        0xE068 => (KeyCode::WWWStop, KeyState::Down),	    0xE0E8 => (KeyCode::WWWStop, KeyState::Up),
        0xE069 => (KeyCode::WWWForward, KeyState::Down),	0xE0E9 => (KeyCode::WWWForward, KeyState::Up),
        0xE06A => (KeyCode::WWWBack, KeyState::Down),   	0xE0EA => (KeyCode::WWWBack, KeyState::Up),
        0xE06B => (KeyCode::MyComputer, KeyState::Down),	0xE0EB => (KeyCode::MyComputer, KeyState::Up),
        0xE06C => (KeyCode::Email, KeyState::Down),     	0xE0EC => (KeyCode::Email, KeyState::Up),
        0xE06D => (KeyCode::MediaSelect, KeyState::Down),	0xE0ED => (KeyCode::MediaSelect, KeyState::Up),
        0xE02AE037 => (KeyCode::PrintScreen, KeyState::Down),	0xE0B7E0AA => (KeyCode::PrintScreen, KeyState::Up),
        0xE11D45E19DC5 => (KeyCode::PauseBreak, KeyState::Down),
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

pub struct Keyboard {
    pub control_flags: u8,
    pub current_byte: u32,
    pub key_map: [(KeyCode, KeyState); 126],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            control_flags: 0,
            current_byte: 0,
            key_map: [(KeyCode::Null, KeyState::Up); 126],
        }
    }

    pub fn read_byte(&mut self) {
        unsafe {
            let scan_code = inb(0x60);
            let full_code = scan_code as u32 | self.current_byte;
            let key_code = scan_to_key(full_code);
            match key_code {
                Some((code, state)) => {
                    self.current_byte = 0;
                    let (current_code, current_state) = self.key_map[code as usize];
                    if state != current_state {
                        self.key_map[code as usize] = (code, state);   
                        /*
                        let raw = vec![("tag", Value::from_str("#keyboard/event/keydown")),
                                       ("key", Value::from_string(format!("{:?}", code)))];
                        let key_event = Entity::from_raw(raw);
                        let txn;
                        if state == KeyState::Down {
                            let changes = key_event.make_changeset(ChangeType::Add);
                            txn = Transaction::from_changeset(changes);
                        } else {
                            let changes = key_event.make_changeset(ChangeType::Remove);
                            txn = Transaction::from_changeset(changes);
                        }
                        MechDB.lock().register_transactions(&mut vec![txn]);
                        */
                        if code == KeyCode::Escape && state == KeyState::Down {
                            SCREEN_WRITER.lock().clear();
                        }
                    }
                },
                None => {
                    self.current_byte = full_code << 8;
                },
            };
            // Tell the keyboard we've read a byte
            outb(0x20, 0x20);
        }
    }
}

lazy_static! {
    pub static ref keyboard: Mutex<Keyboard> = Mutex::new(Keyboard::new());
}
