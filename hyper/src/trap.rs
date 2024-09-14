use core::arch::global_asm;

use riscv::register::stvec;

global_asm!(include_str!("trap.S"));

#[repr(C)]
struct TrapFrame {
    gprs: [usize; 32],
    sstatus: usize,
    sepc: usize,
}

extern "C" {
    fn _trap_entry();
}

pub fn setup_trap_handler() {
    unsafe {
        stvec::write(_trap_entry as usize, stvec::TrapMode::Direct);
    }
}
