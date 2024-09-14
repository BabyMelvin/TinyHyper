#![no_main]
#![no_std]
#![feature(panic_info_message)]

use bit_field::BitField;
use buddy_system_allocator::LockedHeap;
use fdt::Fdt;
use log::{info, LevelFilter};
use logs::HyperLog;
use riscv::{
    hyper::{
        hcounteren,
        hir::hie,
        htdr::{hedeleg, hideleg},
    },
    register::{
        mcause::{Exception, Interrupt},
        scounteren,
        sie::{self, Sie},
        sip::{self, Sip},
        sstatus,
    },
};

extern crate alloc;
use crate::{trap::setup_trap_handler, loader::loader_guest_vm, mm_g::init_mm};

pub mod boot;
pub mod driver;
pub mod logs;
mod panic;
pub mod sbi;
pub mod trap;
pub mod util;
pub mod loader;
pub mod vm;
pub mod mm_g;

extern "C" {
    static _start: u8;
    static _stack_end: u8;
    static mut _sheap: usize;
    static mut _eheap: usize;
    static mut _heap_size: usize;
}

static HYPER_LOG: HyperLog = HyperLog;

#[no_mangle]
extern "C" fn hyper_init(hart_id: u64, fdt_addr: u64) {
    // Log alive
    log::set_logger(&HYPER_LOG).unwrap();
    log::set_max_level(LevelFilter::Debug);
    info!("Hello TinyHper, Boot Hart: {hart_id}");

    init_mm();

    // Now you can use heap.
    init_s_csrs();

    setup_trap_handler();

    unsafe {
        let fdt = Fdt::from_ptr(fdt_addr as *const u8).expect("Failed to open Device tree");
    }

    loader_guest_vm();
}

fn init_s_csrs() {
    sie::write(Sie::default());
    sip::write(Sip::default());
    unsafe {
        sstatus::set_fs(sstatus::FS::Off);
    }

    // delegte traps to VS
    let hedeleg = hedeleg::read();
    hedeleg
        .bits()
        .set_bit(Exception::InstructionMisaligned as usize, true);
    hedeleg
        .bits()
        .set_bit(Exception::IllegalInstruction as usize, true);
    hedeleg.bits().set_bit(Exception::Breakpoint as usize, true);
    hedeleg
        .bits()
        .set_bit(Exception::LoadMisaligned as usize, true);
    hedeleg
        .bits()
        .set_bit(Exception::StoreMisaligned as usize, true);
    hedeleg
        .bits()
        .set_bit(Exception::UserEnvCall as usize, true);
    hedeleg
        .bits()
        .set_bit(Exception::InstructionFault as usize, true);
    hedeleg
        .bits()
        .set_bit(Exception::LoadPageFault as usize, true);
    hedeleg
        .bits()
        .set_bit(Exception::StorePageFault as usize, true);
    hedeleg::write(hedeleg);

    let hideleg = hideleg::read();
    hideleg
        .bits()
        .set_bit(Interrupt::VirtualSupervisorSoft as usize, true);
    hideleg
        .bits()
        .set_bit(Interrupt::VirtualSupervisorTimer as usize, true);
    hideleg
        .bits()
        .set_bit(Interrupt::VirtualSupervisorExternal as usize, true);
    hideleg::write(hideleg);

    let hie = hie::read();
    hie.bits()
        .set_bit(Interrupt::VirtualSupervisorExternal as usize, true);
    hie.bits()
        .set_bit(Interrupt::VirtualSupervisorSoft as usize, true);
    hie.bits()
        .set_bit(Interrupt::VirtualSupervisorTimer as usize, true);
    hie::write(hie);

    let hcounteren = hcounteren::read();
    hcounteren.bits().set_bits(0..=63, 0xFFFF_FFFF_FFFF_FFFF);
    hcounteren::write(hcounteren);

    unsafe {
        // counter for U-mode
        scounteren::set_cy();
        scounteren::set_tm();
        scounteren::set_ir();
    }
}
