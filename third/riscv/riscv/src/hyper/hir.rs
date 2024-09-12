//! HS-mode: decreasing
//! SEI, SSI, STI, SGEI, VSEI, VSSI, VSTI.
pub mod hvip {
    use bit_field::BitField;

    use crate::{read_csr_as, write_csr_as};

    /// `HS-mode` write to indicate `VS-mode`'s virtual interrupts
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Hvip {
        bits: usize,
    }

    impl Hvip {
        /// hvip asserts a VS-level external interrupt
        pub fn set_vseip(&mut self, vseip: bool) {
            self.bits.set_bit(10, vseip);
        }

        /// asserts a VS-level timer interrupt
        pub fn set_vstip(&mut self, vstip: bool) {
            self.bits.set_bit(6, vstip);
        }

        /// asserts a VS-level software interrupt.
        pub fn set_vssip(&mut self, vssip: bool) {
            self.bits.set_bit(2, vssip);
        }
    }

    write_csr_as!(Hvip, 0x645);
    read_csr_as!(Hvip, 0x645);
}

pub mod hip {
    use bit_field::BitField;

    use crate::read_csr_as;

    /// supplement HS-level’s sip
    /// if `sie` bit i read-only zero, same bit of hip may writtable or read-only.
    ///     when hip writtable, pending i write 0 to clear.
    ///     when hip read-only, but i become pending:
    ///         hvip i set clear to clear pending
    ///         or other clearing the pending interrupt machanism
    pub struct Hip {
        bits: usize,
    }

    impl Hip {
        // guest external interrupts at supervisor level (HS-level)
        // Indicate: hgeip.bits & hgeie != 0
        pub fn sgeip(&self) -> bool {
            self.bits.get_bit(12)
        }

        // VS-level external interrupts.
        // Indicate: hvip.VSEIP | hgeip setted by hstatus.VGEIN | directed external interrupt to VS-level
        pub fn vseip(&self) -> bool {
            self.bits.get_bit(10)
        }

        // VS-level timer interrupts.
        // Indicate: hvip.VSTIP | directed timer interrupt to VS-level
        pub fn vstip(&self) -> bool {
            self.bits.get_bit(6)
        }

        // VS-level software interrupts
        // Indicate: same bit in hvip
        pub fn vssip(&self) -> bool {
            self.bits.get_bit(2)
        }
    }
    read_csr_as!(Hip, 0x644);
}

pub mod hie {
    use crate::{read_csr_as, write_csr_as};
    use bit_field::BitField;

    /// supplement HS-level’s sie
    /// The hip register indicates pending VS-level and hypervisor-specific interrupts,
    ///     while hie contains enable bits for the same interrupts
    /// Note: sie and hie mutually exclusive, likewise for sip and hip
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Hie {
        bits: usize,
    }

    impl Hie {
        pub fn trap_in_hs_mode() -> bool {
            /* interrupt trap in HS-mode, all true:
               a. current is HS-mode and sstatus.SIE=1, or mode < HS-mode
               b. sie and sie = 1, or hip and hie = 1
               c. hideleg != 1
            */
            true
        }
        // guest external interrupts at supervisor level (HS-level)
        pub fn sgeip(&self) -> bool {
            self.bits.get_bit(12)
        }

        pub fn set_sgeip(&mut self, sgeip: bool) {
            self.bits.set_bit(12, sgeip);
        }

        // VS-level external interrupts.
        pub fn vseip(&self) -> bool {
            self.bits.get_bit(10)
        }

        pub fn set_vseip(&mut self, vseip: bool) {
            self.bits.set_bit(10, vseip);
        }

        // VS-level timer interrupts.
        pub fn vstip(&self) -> bool {
            self.bits.get_bit(6)
        }

        pub fn set_vstip(&mut self, vstip: bool) {
            self.bits.set_bit(6, vstip);
        }

        // VS-level software interrupts
        pub fn vssip(&self) -> bool {
            self.bits.get_bit(2)
        }

        pub fn set_vssip(&mut self, vssip: bool) {
            self.bits.set_bit(2, vssip);
        }
    }
    read_csr_as!(Hie, 0x604);
    write_csr_as!(Hie, 0x604);
}
