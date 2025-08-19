use core::fmt;

use crate::console::ConsoleWrite;

pub fn _print(args: fmt::Arguments) {
    if let Err(_) = crate::console::Console::get().write(args) {}
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        crate::print::_print(format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        crate::print::_print(format_args!($($arg)*));
        crate::print!("\n");
    }
}
