
pub struct HyperLoader {
    pub name: &'static str,
    pub addr: usize,
    pub rootfs: usize,
    pub sepc: usize,
}

impl HyperLoader {
    pub fn load_from_disk(&mut self) {

    }
}

pub fn loader_guest_vm()
{

}