// # Video Graphics Array (VGA) Buffer


// ## Prelude

use core::fmt;

// print! and println! macros for printing text to the screen.

macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::drivers::vga::print(format_args!($($arg)*));
    });
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}