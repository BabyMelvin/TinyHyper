//! htval is written with additional exception-specific information,
//! alongside stval, to assist software in handling the trap

use crate::read_csr_as;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Htval {
    bits: usize,
}

impl Htval {

    // the guest physical address that faulted, shifted right by 2 bits
    pub fn guest_pa(&self) -> usize {
        self.bits << 2
    }
}

read_csr_as!(Htval, 0x643);