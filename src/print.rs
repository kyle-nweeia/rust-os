use core::fmt;
use core::fmt::Write;

struct Console;

impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        Ok(for c in s.chars() {
            unsafe { core::ptr::write(0x3F201000 as *mut char, c) }
        })
    }
}

pub fn _print(args: fmt::Arguments) {
    if let Err(_) = Console.write_fmt(args) {}
}

#[macro_export]
macro_rules! print {
    ($($arg:tt),*) => {
        $(crate::print::_print(format_args!($arg));)*
    }
}

#[macro_export]
macro_rules! println {
    ($($arg:tt),*) => {
        $(crate::print::_print(format_args!($arg));)*
        crate::print!("\n");
    }
}
