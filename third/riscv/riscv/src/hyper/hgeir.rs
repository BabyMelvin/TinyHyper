//! indicates pending guest external interrupts for this hart
//! Guest external interrupts represent interrupts **directed to**
//!     `individual virtual machines` at VS-level.
//! pass-through or direct between vm and physical device.
//! This number is known as GEILEN.
pub mod hgeip {
    use crate::read_csr_as;


    /// Each bit of hgeip summarizes all pending interrupts directed to one virtual hart
    /// as collected and reported by an interrupt controller.
    ///     SW query interrupt controller to distinguish specific pending interrupt.
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Hgeip {
        bits: usize,
    }

    read_csr_as!(Hgeip, 0xE12);
}

/// Register hgeie selects the subset of guest external interrupts
///     that cause a supervisor-level (HS-level) guest external interrupt.
/// NOTE: The enable bits in hgeie do not affect the VS-level external interrupt signal
///     selected from hgeip by hstatus.VGEIN.
pub mod hgeie {
    use crate::read_csr_as;

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Hgeie {
        bits: usize,
    }
    read_csr_as!(Hgeie, 0x607);
}