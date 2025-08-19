use core::fmt::{Arguments, Result, Write};

use crate::console_data::ConsoleData;
use crate::lock::{Lock, Mutex};

pub struct Console {
    data: Lock<ConsoleData>,
}

pub trait ConsoleStatistics {
    fn chars_written(&self) -> usize;
}

pub trait ConsoleWrite {
    fn write(&self, args: Arguments) -> Result;
}

impl Console {
    pub const fn new() -> Self {
        Self {
            data: Lock::new(ConsoleData::new()),
        }
    }

    pub const fn get() -> &'static Self {
        &CONSOLE
    }
}

impl ConsoleStatistics for Console {
    fn chars_written(&self) -> usize {
        self.data.lock(|data| data.get_chars_written())
    }
}

impl ConsoleWrite for Console {
    fn write(&self, args: Arguments) -> Result {
        self.data.lock(|data| Write::write_fmt(data, args))
    }
}

static CONSOLE: Console = Console::new();
