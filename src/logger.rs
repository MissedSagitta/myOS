use log::{self, LevelFilter};

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool { true }
    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let color = match record.level() {
            log::Level::Error => 31,
            log::Level::Warn => 93,
            log::Level::Info => 34,
            log::Level::Debug => 32,
            log::Level::Trace => 90,
        };
        println!("\u{1B}[{}m[{:>5}] {}\u{1B}[0m",
            color, record.level(), record.args()
        );
    }
    fn flush(&self) {}
}

pub fn log_init() {
    static LOGGER: Logger = Logger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("error") => log::LevelFilter::Error,
        Some("warn") => log::LevelFilter::Warn,
        Some("info") => log::LevelFilter::Info,
        Some("debug") => log::LevelFilter::Debug,
        Some("trace") => log::LevelFilter::Trace,
        _ => LevelFilter::Trace,
    });
}