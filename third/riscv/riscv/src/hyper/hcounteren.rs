use bit_field::BitField;

use crate::{read_csr_as, write_csr_as};

/// CY, TM, IR, or HPMn bit in the hcounteren register is clear:
///     **read** the `cycle`, `time`, `instret`, or `hpmcountern` register
///     while V=1 will cause a virtual
///     instruction exception if the same bit in mcounteren is 1
/// VU-mode readable only when hcounteren & scounteren
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hcounteren {
    // a 32-bit register
    bits: usize,
}

impl Hcounteren {
    pub fn bits(&self) -> usize {
        self.bits
    }

    // cycle
    pub fn cy(&self) -> bool {
        self.bits.get_bit(0)
    }

    // time
    pub fn tm(&self) -> bool {
        self.bits.get_bit(1)
    }

    // insret
    pub fn ir(&self) -> bool {
        self.bits.get_bit(2)
    }

    pub fn hpmx(&self, i: u8) -> bool {
        self.bits.get_bit(i as usize)
    }
}

read_csr_as!(Hcounteren, 0x606);
write_csr_as!(Hcounteren, 0x606);
