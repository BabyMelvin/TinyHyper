//! The hstatus register provides facilities analogous to the mstatus register for tracking and
//! controlling the exception behavior of a `VS-mode guest`.

use bit_field::BitField;

use crate::{
    bits::{bf_extract, bf_insert},
    clear, read_csr_as, set, set_clear_csr, write_csr_as,
};

#[derive(Clone, Copy, Debug)]
pub struct Hstatus {
    bits: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VSXL {
    VSXL32 = 1,
    VSXL64 = 2,
    VSXL128 = 3,
}

impl From<usize> for Hstatus {
    fn from(value: usize) -> Self {
        Hstatus { bits: value }
    }
}

impl Hstatus {
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Updated SRet, cause virtual instruction exception
    pub fn vtsr(&self) -> bool {
        bf_extract(self.bits, 22, 1) != 0
    }

    pub fn set_vtsr(&mut self, sr: bool) {
        self.bits = bf_insert(self.bits, 22, 1, sr as usize);
    }

    /// Updatea wfi, cause virtual instruction exception
    pub fn vtw(&self) -> bool {
        bf_extract(self.bits, 21, 1) != 0
    }

    pub fn set_vtw(&mut self, tw: bool) {
        self.bits = bf_insert(self.bits, 21, 1, tw as usize);
    }

    /// Update VM, exe SFENCE.VMA or SINVAL.VMA cause virtual instruction exception
    pub fn vtvm(&self) -> bool {
        bf_extract(self.bits, 20, 1) != 0
    }

    pub fn set_vtvm(&mut self, vm: bool) {
        self.bits = bf_insert(self.bits, 20, 1, vm as usize)
    }

    /// VGEIN Virtual Guest External Interrupt Number
    /// a guest external interrupt source for VS-level external interrupts.
    pub fn get_vgein(&self) -> u8 {
        self.bits.get_bits(12..=17) as u8
    }

    pub fn set_vgein(&mut self, vgein: usize) {
        self.bits.set_bits(12..=17, vgein);
    }

    /// Hypervisor in U-mode
    /// whether the virtual-machine `load/store` instructions,
    /// `HLV`, `HLVX`, and `HSV`, can be used also in U-mode
    pub fn hu(&self) -> bool {
        self.bits.get_bit(9)
    }
    pub fn set_hu(&mut self, hu: bool) {
        self.bits.set_bit(9, hu);
    }

    /// Supervisor Previous Virtualization mode as SPP
    /// SRET when V=0 exec, V is set to SPV
    pub fn spv(&self) -> bool {
        self.bits.get_bit(7)
    }

    pub fn set_spv(&mut self, spv: bool) {
        self.bits.set_bit(7, spv);
    }

    /// Supervisor Previous Virtual Privilege
    /// When V=1 trap into HS-mode,spvp set as sstatus.SPP
    ///     V=0 trap into HS-mode, spvp not changed.
    /// SPVP control(HLV, HLVX and HSV) the effective privilege of explicit memory accesses
    pub fn spvp(&self) -> bool {
        self.bits.get_bit(8)
    }

    pub fn set_spvp(&mut self, spvp: bool) {
        self.bits.set_bit(8, spvp);
    }

    /// Guest Virtual Address
    /// trap write virtual address to stval, GVA = 1:
    ///     breakpoint, address misaligned, access fault, page fault, or guestpage fault
    /// others: GVA = 0
    pub fn gva(&self) -> bool {
        self.bits.get_bit(6)
    }

    /// VS-mode endianness of explicit memory accesses
    ///  VSBE = 0, little endian
    ///  VSBE = 1, big endian
    pub fn vsbe(&self) -> bool {
        self.bits.get_bit(5)
    }
}

read_csr_as!(Hstatus, 0x600);
write_csr_as!(Hstatus, 0x600);

// csrrs & csrrc assemblely
set!(0x600);
clear!(0x600);
