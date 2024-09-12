//! htinst is written with a value that, if nonzero, provides information about the instruction that trapped,
//! to assist software in handling the trap

use crate::read_csr_as;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Htinst {
    bits: usize,
}

read_csr_as!(Htinst, 0x64A);