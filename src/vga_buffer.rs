// # Video Graphics Array (VGA) Buffer


// ## Prelude

use core::fmt;
use volatile::Volatile;
use core::ptr::Unique;
use spin::Mutex;

// ## Constants

// We know the VGA rendering area is 25 x 80, and rests at memory location 
// 0xb8000

const VGA_HEIGHT: usize = 25;
const VGA_WIDTH: usize = 80;
pub const VGA_ADDRESS: usize = 0xb8000;

// ## Color

// There are 16 colors we can work with.

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

// A Color code consists of an 8 bit foreground and 8 bit background color.
// The High bits are the background, while the Low bits are the foreground.

#[derive(Debug, Clone, Copy)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// A Character on the Screen has a character and color code which tells us how
// to draw it.

#[derive(Debug, Clone, Copy)]
#[repr(C)] // Lay out struct as in C for correct field ordering.
struct ScreenCharacter {
    ascii_character: u8,
    color_code: ColorCode,
}

// We represent the screen as a 25 x 80 buffer of ScreenCharacters.

// Volatile tells rust that there are side effects. We use it to make sure Rust
// does not optimize away printing screen characters, which are the side 
// effect.

struct Buffer {
    chars: [[Volatile<ScreenCharacter>; VGA_WIDTH]; VGA_HEIGHT],
}

// Create a screen writer type that writes character bytes to a buffer.

pub struct ScreenWriter {
    x: usize, 
    y: usize,
    color_code: ColorCode,
    buffer: Unique<Buffer>,
}

impl ScreenWriter {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            // Move to a new line on a new line char
            b'\n' => self.new_line(),
            // Place a char in the screen buffer for any other byte
            byte => {
                // Insert a new line if we've reached the end of the screen
                if self.x >= VGA_WIDTH {
                    self.new_line();
                }
                let row = self.y;
                let col = self.x;
                let color_code = self.color_code;

                // Place the character into the buffer at the position (row,col)
                self.buffer().chars[row][col].write(ScreenCharacter {
                  ascii_character: byte,
                  color_code: color_code,  
                });

                self.x += 1;
            }
        }
    }

    pub fn write_char(&mut self, character: char) {
        self.write_byte(character as u8);
    }

    // Converts raw pointer to a safe buffer reference
    fn buffer(&mut self) -> &mut Buffer {
        unsafe{ self.buffer.as_mut() }
    }

    // Move every character up a row, and clear the last row.
    fn new_line(&mut self) { 
        for row in 1..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                let buffer = self.buffer();
                let character = buffer.chars[row][col].read();
                buffer.chars[row - 1][col].write(character);
            }
        }
        let row_ix = self.y;
        self.clear_row(row_ix);
        self.x = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenCharacter {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        // Write a blank char across the row
        for col in 0..VGA_WIDTH {
            self.buffer().chars[row][col].write(blank);
        }
    }
}

// We implement a fmt:Write for the ScreenWriter to support rust macros

impl fmt::Write for ScreenWriter {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        for byte in string.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

// Instantiate a screen writer. We use a spin lock to avoid race conditions.

pub static SCREEN_WRITER: Mutex<ScreenWriter> = Mutex::new(ScreenWriter {
    x: 0,
    y: VGA_HEIGHT - 1,
    color_code: ColorCode::new(Color::LightBlue, Color::Black),
    buffer: unsafe { Unique::new_unchecked(VGA_ADDRESS as *mut _) },
});

// And now we have everything we need to implement the print! and println! macros

macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::vga_buffer::print(format_args!($($arg)*));
    });
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

// Evaluate arguments before locking the SCREEN_WRITER to avoid a deadlock
// e.g. println!("{}", { println!("inner"); "outer" });
pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    SCREEN_WRITER.lock().write_fmt(args).unwrap();
}

// A utility function to clear the screen of all characters
pub fn clear_screen() {
    for _ in 0..VGA_HEIGHT {
        println!("");
    }
}