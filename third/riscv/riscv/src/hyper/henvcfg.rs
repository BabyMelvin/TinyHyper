//! henvcfg controls certain characteristics of the execution environment
//! when virtualization mode V=1

use bit_field::BitField;

use crate::{read_csr_as, write_csr_as};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Henvcfg {
    bits: usize,
}

impl Henvcfg {
    /// (Fence of I/O implies Memory)
    /// requirement to I/O also implies to main memory
    ///     FIOM=1 and V=1: attomic instruction also set.
    ///     PI => Predecessor device input and memory reads (PR implied)
    ///     PO => Predecessor device output and memory writes (PW implied)
    ///     SI => Successor device input and memory reads (SR implied)
    ///     SO => Successor device output and memory writes (SW implied)
     pub fn fiom(&self) -> bool {
        self.bits.get_bit(0)
     }

     pub fn set_fiom(&mut self, fiom: bool) {
        self.bits.set_bit(0, fiom);
     }

    /// whether the Svpbmt extension is available
    /// `Page-Based Memory Types` override the PMA(s) for the associated memory pages
     pub fn pbmte(&self) -> bool {
        self.bits.get_bit(62)
     }
}

read_csr_as!(Henvcfg, 0x60A);
write_csr_as!(Henvcfg, 0x60A);