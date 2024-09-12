
pub mod hideleg{
    use crate::{read_csr_as, write_csr_as};
    /// When a virtual supervisor external interrupt (code 10) is delegated to VS-mode,
    /// it is automatically translated by the machine
    /// into a supervisor external interrupt (code 9) for `VS-mode`
    ///     and write value to `vscause`
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Hideleg {
        bits: usize,
    }

    read_csr_as!(Hideleg, 0x603);
    write_csr_as!(Hideleg, 0x603);
}

pub mod hedeleg {
    use crate::{read_csr_as, write_csr_as};

    /// The hedeleg and hideleg CSRs allow
    /// these traps to be further delegated to a VS-mode guest
    /// A synchronous trap that has been delegated to HS-mode (using medeleg)
    /// is further delegated to `VS-mode` if V=1
    /// before the trap and the corresponding hedeleg bit is set.
    /// Table 8.2 referring.
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Hedeleg {
        bits: usize,
    }

    read_csr_as!(Hedeleg, 0x602);
    write_csr_as!(Hedeleg, 0x602);
}