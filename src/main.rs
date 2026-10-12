// myCore/src/main.rs
#![no_main]
#![no_std]

mod lang_items;
mod sbi;

#[macro_use]
mod console;
mod logger;

use core::arch::global_asm;
use log::{debug, error, info, trace, warn};

global_asm!(include_str!("entry.asm"));

macro_rules! linker_symbol_addr {
    ($symbol:path) => {
        ($symbol as *const ()).addr()
    };
}

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    core_init();
    println!("Hello, world!");
    sbi::shutdown(false);
}

fn core_init() {
    clear_bss();
    logger::log_init();
    info!("Initializing...");
    info!("Log level initialized to {}", log::max_level());

    info!("Initialization completed.");
    let separators = "*****************************";
    info!("{}", separators);
    info!("Welcome to MyCore");
    info!("{}", separators);
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
