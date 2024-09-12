pub mod hstatus;

// Hypervisor Trap Delegation Registers (hedeleg and hideleg)
pub mod htdr;

// Hypervisor Interrupt Registers (hvip, hip, and hie)
pub mod hir;

// Hypervisor Guest External Interrupt Registers (hgeip and hgeie)
pub mod hgeir;

pub mod henvcfg;
pub mod hcounteren;
pub mod htimedelta;
pub mod htval;
pub mod htinst;
pub mod hgatp;
pub mod vs;
pub mod asm;

pub mod mtval2;
pub mod mtinst;