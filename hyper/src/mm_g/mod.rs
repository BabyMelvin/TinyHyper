//! G-stage translation and protection

use alloc::{boxed::Box, vec::Vec};
use buddy_system_allocator::LockedHeap;
use lazy_static::lazy_static;
use spin::{mutex::Mutex, once::Once};

use self::{
    address::PhysPageNum,
    frame::{FrameAllocatorImpl, FrameTracker, StackFrameAllocator},
};

pub mod address;
pub mod frame;
pub mod memory_set;
pub mod page_table;

pub const HYPER_HEAP_SIZE: usize = 8192 * 1024;
#[no_mangle]
#[link_section = ".heap"]
static mut HYPER_HEAP_SPACE: [u8; HYPER_HEAP_SIZE] = [0; HYPER_HEAP_SIZE];

#[global_allocator]
static HYPER_HEAP: LockedHeap<32> = LockedHeap::<32>::empty();

fn init_hyper_heap() {
    unsafe {
        HYPER_HEAP
            .lock()
            .init(HYPER_HEAP_SPACE.as_ptr() as usize, HYPER_HEAP_SIZE);
    }
}
static MM_G_INIT: Once = Once::new();

trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn alloc_more(&mut self, pages: usize) -> Option<Vec<PhysPageNum>>;
    fn dealloc(&mut self, ppn: PhysPageNum);
}

lazy_static! {
    pub static ref FRAME_ALLOCATOR: Mutex<Option<Box<FrameAllocatorImpl>>> = Mutex::new(None);
}

pub fn init_mm() {
    MM_G_INIT.call_once(|| {
        init_hyper_heap();
        *FRAME_ALLOCATOR.lock() = Some(Box::new(FrameAllocatorImpl::new()));
        init_frame_allocator();
    });
}

pub fn init_frame_allocator() {}

pub fn frame_alloc() -> Option<FrameTracker> {
    FRAME_ALLOCATOR
        .lock()
        .as_mut()
        .unwrap()
        .alloc()
        .map(FrameTracker::new)
}

pub fn frame_alloc_more(num: usize) -> Option<Vec<FrameTracker>> {
    FRAME_ALLOCATOR
        .lock()
        .as_mut()
        .unwrap()
        .alloc_more(num)
        .map(|x| x.iter().map(|&t| FrameTracker::new(t)).collect())
}

pub fn frame_dealloc(ppn: PhysPageNum) {
    FRAME_ALLOCATOR.lock().as_mut().unwrap().dealloc(ppn);
}
