#![no_main]
#![no_std]

mod console;
mod console_data;
mod lock;
mod print;

use crate::console::ConsoleStatistics;

fn main() -> ! {
    println!("Hello, world!");
    println!(
        "Characters written: {}",
        crate::console::Console::get().chars_written()
    );
    panic!();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(export_name = "_start")]
#[unsafe(naked)]
unsafe extern "C" fn start() {
    core::arch::naked_asm!(
        "mov sp, #0x80000",
        "b {main}",
        main = sym main,
    );
}
