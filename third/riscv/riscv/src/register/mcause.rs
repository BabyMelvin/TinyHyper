//! mcause register

/// mcause register
#[derive(Clone, Copy, Debug)]
pub struct Mcause {
    bits: usize,
}

impl From<usize> for Mcause {
    #[inline]
    fn from(bits: usize) -> Self {
        Self { bits }
    }
}

/// Trap Cause
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Trap {
    Interrupt(Interrupt),
    Exception(Exception),
}

/// Interrupt
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Interrupt {
    UserSoft = 0,
    SupervisorSoft = 1,
    VirtualSupervisorSoft = 2,
    MachineSoft = 3,
    UserTimer = 4,
    SupervisorTimer = 5,
    VirtualSupervisorTimer = 6,
    MachineTimer = 7,
    UserExternal = 8,
    SupervisorExternal = 9,
    VirtualSupervisorExternal = 10,
    MachineExternal = 11,
    SupervisorGuestExternal = 12,
    Unknown,
}

/// Exception
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Exception {
    InstructionMisaligned = 0,
    InstructionFault = 1,
    IllegalInstruction = 2,
    Breakpoint = 3,
    LoadMisaligned = 4,
    LoadFault = 5,
    StoreMisaligned = 6,
    StoreFault = 7,
    UserEnvCall = 8,
    SupervisorEnvCall = 9,
    VirtualSupervisorEnvCall = 10,
    MachineEnvCall = 11,
    InstructionPageFault = 12,
    LoadPageFault = 13,
    StorePageFault = 15,
    GuestInstructionPageFault = 20,
    GuestLoadPageFault = 21,
    VirtualInstruction = 22,
    GuestStorePageFault = 23,
    Unknown,
}

impl From<usize> for Interrupt {
    #[inline]
    fn from(nr: usize) -> Self {
        match nr {
            0 => Self::UserSoft,
            1 => Self::SupervisorSoft,
            2 => Self::VirtualSupervisorSoft,
            3 => Self::MachineSoft,
            4 => Self::UserTimer,
            5 => Self::SupervisorTimer,
            6 => Self::VirtualSupervisorTimer,
            7 => Self::MachineTimer,
            8 => Self::UserExternal,
            9 => Self::SupervisorExternal,
            10 => Self::VirtualSupervisorExternal,
            11 => Self::MachineExternal,
            12 => Self::SupervisorGuestExternal,
            _ => Self::Unknown,
        }
    }
}

impl TryFrom<Interrupt> for usize {
    type Error = Interrupt;

    #[inline]
    fn try_from(value: Interrupt) -> Result<Self, Self::Error> {
        match value {
            Interrupt::Unknown => Err(Self::Error::Unknown),
            _ => Ok(value as Self),
        }
    }
}

impl From<usize> for Exception {
    #[inline]
    fn from(nr: usize) -> Self {
        match nr {
            0 => Self::InstructionMisaligned,
            1 => Self::InstructionFault,
            2 => Self::IllegalInstruction,
            3 => Self::Breakpoint,
            4 => Self::LoadMisaligned,
            5 => Self::LoadFault,
            6 => Self::StoreMisaligned,
            7 => Self::StoreFault,
            8 => Self::UserEnvCall,
            9 => Self::SupervisorEnvCall,
            10 => Self::VirtualSupervisorEnvCall,
            11 => Self::MachineEnvCall,
            12 => Self::InstructionPageFault,
            13 => Self::LoadPageFault,
            15 => Self::StorePageFault,
            20 => Self::GuestInstructionPageFault,
            21 => Self::GuestLoadPageFault,
            22 => Self::VirtualInstruction,
            23 => Self::GuestStorePageFault,
            _ => Self::Unknown,
        }
    }
}

impl TryFrom<Exception> for usize {
    type Error = Exception;

    #[inline]
    fn try_from(value: Exception) -> Result<Self, Self::Error> {
        match value {
            Exception::Unknown => Err(Self::Error::Unknown),
            _ => Ok(value as Self),
        }
    }
}

impl Mcause {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Returns the code field
    #[inline]
    pub fn code(&self) -> usize {
        self.bits & !(1 << (usize::BITS as usize - 1))
    }

    /// Trap Cause
    #[inline]
    pub fn cause(&self) -> Trap {
        if self.is_interrupt() {
            Trap::Interrupt(Interrupt::from(self.code()))
        } else {
            Trap::Exception(Exception::from(self.code()))
        }
    }

    /// Is trap cause an interrupt.
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        self.bits & (1 << (usize::BITS as usize - 1)) != 0
    }

    /// Is trap cause an exception.
    #[inline]
    pub fn is_exception(&self) -> bool {
        !self.is_interrupt()
    }
}

read_csr_as!(Mcause, 0x342);
