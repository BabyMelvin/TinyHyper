//! controls `G-stage` address translation and protection
//! Note: the root page table is 16 KiB and must be aligned to a 16-KiB boundary

use bit_field::BitField;

use crate::{read_csr_as, write_csr_as};
pub struct Hgatp {
    bits: usize,
}

pub enum MODE {
    BARE = 0,
    SV39x4 = 8,
    SV48x4 = 9,
    SV57x4 = 10,
}

impl Hgatp {
    // the physical page number (PPN) of the guest-physical root page table
    pub fn ppn(&self) -> usize {
        self.bits.get_bits(0..=22)
    }

    // a virtual machine identifier (VMID), which facilitates address-translation fences
    //   on a per-virtual-machine basis
    pub fn vmid(&self) -> usize {
        self.bits.get_bits(44..=57)
    }

    // selects the address-translation scheme for guest physical addresses
    pub fn mode(&self) -> usize {
        self.bits.get_bits(60..=63)
    }
}

read_csr_as!(Hgatp, 0x680);
write_csr_as!(Hgatp, 0x680);
