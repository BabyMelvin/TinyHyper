use core::{arch::asm, mem::MaybeUninit};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SbiRet {
    pub err_code: isize,
    pub ret_value: isize,
}

pub mod eid {
    #![allow(missing_docs, dead_code)]
    pub const SBI_EXT_0_1_SET_TIMER: usize = 0x0;
    pub const SBI_EXT_0_1_CONSOLE_PUTCHAR: usize = 0x1;
    pub const SBI_EXT_0_1_CONSOLE_GETCHAR: usize = 0x2;
    pub const SBI_EXT_0_1_CLEAR_IPI: usize = 0x3;
    pub const SBI_EXT_0_1_SEND_IPI: usize = 0x4;
    pub const SBI_EXT_0_1_REMOTE_FENCE_I: usize = 0x5;
    pub const SBI_EXT_0_1_REMOTE_SFENCE_VMA: usize = 0x6;
    pub const SBI_EXT_0_1_REMOTE_SFENCE_VMA_ASID: usize = 0x7;
    pub const SBI_EXT_0_1_SHUTDOWN: usize = 0x8;
    // Extension constants
    pub const EXT_PUT_CHAR: usize = 0x01;
    pub const EXT_BASE: usize = 0x10;
    pub const EXT_HART_STATE: usize = 0x48534D;
    pub const EXT_RESET: usize = 0x53525354;
    pub const EXT_TEE: usize = 0x41544545;
    pub const EXT_MEASUREMENT: usize = 0x5464545;
    // TODO Replace the measurement extension once the `GetEvidence` implementation is complete
    pub const EXT_ATTESTATION: usize = 0x41545354; // ATST
}

#[derive(Debug, Clone, Copy)]
pub struct SbiCall {
    pub eid: usize,
    pub fid: usize,
}

impl From<SbiRet> for isize {
    fn from(value: SbiRet) -> Self {
        if value.err_code == 0 {
            return 0;
        }

        value.ret_value
    }
}

pub unsafe fn sbi_ecall(which: SbiCall, args: [usize; 6]) -> isize {
    let mut ret = MaybeUninit::<SbiRet>::zeroed().assume_init();

    asm!("ecall",
        inlateout("a0") args[0] => ret.err_code,
        inlateout("a1") args[1] => ret.ret_value,
        in("a2") args[2], in("a3") args[3],
        in("a4") args[4], in("a5") args[5],
        in("a6") which.fid, in("a7") which.eid, options(nostack)
    );

    ret.into()
}
