use core::{arch::global_asm, fmt::Error};

use crate::{memlayout, println, uart, print};

global_asm!(include_str!("kernel.S"));

extern "C" {
    #[link_name = "trap_to_kernel"]
    pub fn trap();
}

#[no_mangle]
pub extern "C" fn rust_entrypoint() -> ! {
    if let Err(e) = init() {
        panic!("failed to initialize. {:?}", e);
    };

    println!("hello world from a guest");
    loop {}
}

pub fn init() -> Result<(), Error> {
    // init UART
    uart::Uart::new(memlayout::UART_BASE).init();

    // leave
    Ok(())
}

#[no_mangle]
pub extern "C" fn rust_trap_handler() {
    log::info!("trapped!");
}
