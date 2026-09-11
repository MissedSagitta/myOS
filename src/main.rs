// myCore/src/main.rs
#![no_main]
#![no_std]

mod lang_items;
mod sbi;

#[macro_use]
mod console;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

macro_rules! linker_symbol_addr {
    ($symbol:path) => {
        ($symbol as *const ()).addr()
    };
}

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello, world!");
    sbi::shutdown(false);
}

fn clear_bss() {
    unsafe extern "C" {
        safe fn sbss();
        safe fn ebss();
    }
    (linker_symbol_addr!(sbss)..linker_symbol_addr!(ebss)).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}
