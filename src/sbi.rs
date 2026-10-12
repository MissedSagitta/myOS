use log::{error, info};

// myCore/src/sbi.rs
pub fn console_putchar(c: usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}

pub fn shutdown(failure: bool) -> ! {
    use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
    if !failure {
        info!("Normal End");
        system_reset(Shutdown, NoReason);
    } else {
        error!("Error End");
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}