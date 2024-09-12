//! Hypervisor Instructions
//! hypervisor adds `virtual-machine load and store instructions` and `two privileged fence instructions`

use core::arch::asm;

/// Invalid in M/HS/U when hstatus.HU=1
/// hstatus.SPVP control access privilege
pub mod vm_load_store {
    // HLV.B, HLV.BU, HLV.H, HLV.HU, HLV.W, HLV.WU, and HLV.D.
    pub fn load(){}

    // HSV.B, HSV.H, HSV.W, and HSV.D
    // HLV.WU, HLV.D, and HSV.D for RV64
    pub fn store(){}
}

/// HFENCE.VVMA is valid only in **M-mode** or **HS-mode**
/// Its effect is much the same as temporarily entering VS-mode
/// and executing SFENCE.VMA.
pub unsafe fn hfence_vvma(asid: usize, vaddr: usize) {
    // HFENCE.VVMA apply a single vm, identified by hgatp.VMID
    asm!("hfence.vvma {0}, {1}", in(reg) vaddr, in(reg) asid);
}

/// HS-mode when mstatus.TVM=0, or M-mode
/// gaddr:a single guest physical address, shifted right by 2 bits
/// vmid: a single virtual machine identifier (VMID)
pub unsafe fn hfence_gvma(vmid: usize, gaddr: usize) {
    asm!("hfence.gvma {0}, {1}", in(reg) gaddr, in(reg) vmid);
}
