use crate::config::{KERNEL_STACK_SIZE, USER_STACK_SIZE};
use crate::trap::TrapContext;
use core::mem::size_of;

#[repr(align(4096))]
#[derive(Copy, Clone)]
pub struct KernelStack {
    pub data: [u8; KERNEL_STACK_SIZE],
}

impl KernelStack {
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    pub fn push_context(&self, trap_ctx: TrapContext) -> usize {
        let trap_ctx_ptr = (self.get_sp() - size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *trap_ctx_ptr = trap_ctx;
        }
        trap_ctx_ptr as usize
    }
}

#[repr(align(4096))]
#[derive(Copy, Clone)]
pub struct UserStack {
    pub data: [u8; USER_STACK_SIZE],
}

impl UserStack {
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}
