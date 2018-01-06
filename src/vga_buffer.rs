#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black       =   0,
    Blue        =   1,
    Green       =   2,
    Cyan        =   3,
    Red         =   4,
    Magenta     =   5,
    Brown       =   6,
    LightGray   =   7,
    DarkGray    =   8,
    LightBlue   =   9,
    LightGreen  =   10,
    LightCyan   =   11,
    LightRed    =   12,
    Pink        =   13,
    Yellow      =   14,
    White       =   15,
}

// A Color code consists of an 8 bit foreground and background color.

#[derive(Debug, Clone, Copy)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// We represent the screen as a matrix of characters 80 wide by 25 height

#[derive(Debug, Clone, Copy)]
#[repr(C)] // Lay out struct as in C for correct field ordering.
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// Volatile tells rust that there are side effects. We use it to make sure Rust
// does not optimize away printing screen characters, which are the side 
// effect.

use volatile::Volatile;

struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// Create a writer type that writes to the screen.

use core::ptr::Unique;

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: Unique<Buffer>,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            // Move to a new line on a new line char
            b'\n' => self.new_line(),
            // Place a char in the screen buffer on for any other byte
            byte => {
                // Insert a new line if we've reached the end of the screen
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code;

                // Place the character into the buffer at the position (row,col)
                self.buffer().chars[row][col].write(ScreenChar {
                  ascii_character: byte,
                  color_code: color_code,  
                });

                self.column_position += 1;
            }
        }
    }

    // Converts raw pointer to a safe buffer reference
    fn buffer(&mut self) -> &mut Buffer {
        unsafe{ self.buffer.as_mut() }
    }

    fn new_line(&mut self) { 
        /* TODO */

    }
}

pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::LightBlue, Color::Black),
        buffer: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
    };
    writer.write_byte(b'Q');
}