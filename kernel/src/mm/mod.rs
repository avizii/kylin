use crate::mm::memory_set::KERNEL_SPACE;

mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}
