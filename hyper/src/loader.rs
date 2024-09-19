use log::info;

use crate::{
    memlayout::{self, GUEST01_DRAM_END, GUEST01_DRAM_START, GUEST_UART_BASE},
    mm_g::{
        memory_set::{MapArea, MapPermission, MapType, MemoySet},
        page_table::PageTable,
    },
};

pub struct HyperLoader {
    pub name: &'static str,
    // pub addr: usize,
    // pub rootfs: usize,
    pub sepc: usize,
}

impl HyperLoader {
    pub fn new(name: &'static str) -> Self {
        HyperLoader {
            name: name,
            sepc: memlayout::GUEST01_DRAM_START,
        }
    }

    pub fn load_from_disk(&mut self) {
        let mut memset = MemoySet::new_bare();

        // create an identify map for Uart MMIO
        memset.push(
            MapArea::new(
                (GUEST_UART_BASE).into(),
                (GUEST_UART_BASE + 0x200).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::X,
            ),
            None,
        );

        // allocating new pages and map GUEST_DRAM_START ~ GUEST_DRAM_END
        // into those pages for guest kernel
        memset.push(
            MapArea::new(
                GUEST01_DRAM_START.into(),
                GUEST01_DRAM_END.into(),
                MapType::Framed,
                MapPermission::R | MapPermission::X,
            ),
            None,
        );
    }
}

pub fn loader_guest_vm() {
    let vm_name = "guest01";
    info!("let create a vm: {vm_name}");
}
