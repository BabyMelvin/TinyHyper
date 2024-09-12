//！ register contains the delta between the value of the time CSR and the value
//!     returned in VS-mode or VU-mode
//!  time value readed from VS/VU-mode = actual time + htimedelta

use crate::read_csr_as;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Htimedelta {
    bits: usize,
}

read_csr_as!(Htimedelta, 0x605);