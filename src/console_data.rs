use core::fmt;

pub struct ConsoleData {
    chars_written: usize,
}

impl ConsoleData {
    pub const fn new() -> Self {
        Self { chars_written: 0 }
    }

    pub fn get_chars_written(&self) -> usize {
        self.chars_written
    }

    fn write_char(&mut self, c: char) {
        unsafe {
            core::ptr::write_volatile(0x3F201000 as *mut char, c);
        }
        self.chars_written += 1;
    }
}

impl fmt::Write for ConsoleData {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        Ok(for c in s.chars() {
            self.write_char(c);
        })
    }
}
