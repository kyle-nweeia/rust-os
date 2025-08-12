#![no_main]
#![no_std]

mod print;

fn main() -> ! {
    println!("Hello, world!");
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
