#![no_main]
#![no_std]
#![feature(panic_info_message)]

use log::{LevelFilter, info};
use logs::HyperLog;

pub mod boot;
pub mod riscv;
pub mod driver;
pub mod sbi;
pub mod util;
pub mod logs;
mod panic;

extern "C" {
    static _start: u8;
    static _stack_end: u8;
}

static HYPER_LOG: HyperLog = HyperLog;

#[no_mangle]
extern "C" fn hyper_init(hart_id: u64, fdt_addr: u64)
{
    // Log alive
    log::set_logger(&HYPER_LOG).unwrap();
    log::set_max_level(LevelFilter::Debug);

    info!("Hello TinyHper");
}