use core::fmt::Write;

use log::Level;

use crate::sbi::{eid::{EXT_BASE, SBI_EXT_0_1_CONSOLE_PUTCHAR}, sbi_ecall, SbiCall};

pub struct HyperLog;

impl Write for HyperLog {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        console_write_bytes(s.as_bytes());
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {
        {
            HyperLog::write_fmt(format_args!($($args)*))
        }
    };
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        {
            use core::fmt::Write;
            let _ = HyperLog::write_fmt(&mut HyperLog, core::format_args!(core::concat!($fmt, "\r\n") $(, $($arg)+)?));
        }
    };
}

impl log::Log for HyperLog {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!("[TinyHyper] {} {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn console_write_bytes(bytes: &[u8]) {
    let base_call = SbiCall {
        eid: SBI_EXT_0_1_CONSOLE_PUTCHAR,
        fid: 0,
    };

    let mut args = [0; 6];

    for &b in bytes {
        args[0] = b as usize;

        unsafe {
            sbi_ecall(base_call, args);
        }
    }
}
