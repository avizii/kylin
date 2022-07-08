use crate::config::MEMORY_END;
use crate::mm::address::{PhysicAddress, PhysicPageNum};
use crate::sync::UPSafeCell;
use alloc::vec::Vec;
use lazy_static::lazy_static;

trait FrameAllocator {
    fn new() -> Self;

    /// 物理页帧分配
    fn alloc(&mut self) -> Option<PhysicPageNum>;

    /// 物理页帧回收
    fn dealloc(&mut self, ppn: PhysicPageNum);
}

pub struct StackFrameAllocator {
    // 空闲内存的起始物理页号
    current: usize,
    // 空闲内存的结束物理页号
    end: usize,
    // 已分配被回收的物理页号
    recycled: Vec<usize>,
}

impl FrameAllocator for StackFrameAllocator {
    fn new() -> Self {
        Self {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }

    fn alloc(&mut self) -> Option<PhysicPageNum> {
        // 检查栈 recycled 内是否存在回收的物理页号，有则直接弹出使用
        if let Some(ppn) = self.recycled.pop() {
            Some(ppn.into())
        } else {
            // 判断是否内存耗尽
            if self.current == self.end {
                None
            } else {
                // 从未分配过的物理页号区间上进行分配
                self.current += 1;
                Some((self.current - 1).into())
            }
        }
    }

    fn dealloc(&mut self, ppn: PhysicPageNum) {
        let ppn = ppn.0;

        // 回收校验：
        //   1.ppn被分配过，物理页号 < current
        //   2.ppn未被回收，物理页号不能在 recycled 中找到
        if ppn >= self.current || (self.recycled.iter().find(|&v| *v == ppn).is_some()) {
            panic!("Frame ppn={:#x} has not been allocated!", ppn);
        }

        self.recycled.push(ppn);
    }
}

impl StackFrameAllocator {
    // 初始化为可用物理页号区间
    pub fn init(&mut self, l: PhysicPageNum, r: PhysicPageNum) {
        self.current = l.0;
        self.end = r.0;
    }
}

type FrameAllocatorImpl = StackFrameAllocator;

lazy_static! {
    pub static ref FRAME_ALLOCATOR: UPSafeCell<FrameAllocatorImpl> =
        unsafe { UPSafeCell::new(FrameAllocatorImpl::new()) };
}

// 初始化页帧管理器
pub fn init_frame_allocator() {
    extern "C" {
        fn ekernel();
    }

    // 物理地址[ekernel ~ MEMORY_END] 分别下/上取整获得可用的物理页号区间
    let start = PhysicAddress::from(ekernel as usize).ceil();
    let end = PhysicAddress::from(MEMORY_END).floor();

    // 物理页帧全局管理器初始化
    FRAME_ALLOCATOR.exclusive_access().init(start, end);
}

pub struct FrameTracker {
    pub ppn: PhysicPageNum,
}

impl FrameTracker {
    pub fn new(ppn: PhysicPageNum) -> Self {
        let bytes_array = ppn.get_bytes_array();
        for i in bytes_array {
            *i = 0;
        }
        Self { ppn }
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        frame_dealloc(self.ppn);
    }
}

pub fn frame_alloc() -> Option<FrameTracker> {
    FRAME_ALLOCATOR
        .exclusive_access()
        .alloc()
        .map(|ppn| FrameTracker::new(ppn))
}

pub fn frame_dealloc(ppn: PhysicPageNum) {
    FRAME_ALLOCATOR.exclusive_access().dealloc(ppn);
}
