use crate::config::{
    APP_BASE_ADDRESS, APP_SIZE_LIMIT, KERNEL_STACK_SIZE, MAX_APP_NUM, USER_STACK_SIZE,
};
use crate::stack::{KernelStack, UserStack};
use crate::trap::TrapContext;
use core::arch::asm;
use core::slice::{from_raw_parts, from_raw_parts_mut};

pub fn get_app_num() -> usize {
    extern "C" {
        fn _num_app();
    }
    unsafe { (_num_app as usize as *const usize).read_volatile() }
}
pub fn load_apps() {
    extern "C" {
        fn _num_app();
    }

    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_app_num();
    let app_start = unsafe { from_raw_parts(num_app_ptr.add(1), num_app + 1) };

    unsafe {
        asm!("fence.i");
    }

    for i in 0..num_app {
        let base_i = get_base_i(i);
        ((base_i)..(base_i + APP_SIZE_LIMIT))
            .for_each(|addr| unsafe { (addr as *mut u8).write_volatile(0) });
        let src =
            unsafe { from_raw_parts(app_start[i] as *const u8, app_start[i + 1] - app_start[i]) };
        let dst = unsafe { from_raw_parts_mut(base_i as *mut u8, src.len()) };
        dst.copy_from_slice(src);
    }
}

fn get_base_i(app_id: usize) -> usize {
    APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT
}

static KERNEL_STACKS: [KernelStack; MAX_APP_NUM] = [KernelStack {
    data: [0; KERNEL_STACK_SIZE],
}; MAX_APP_NUM];

static USER_STACKS: [UserStack; MAX_APP_NUM] = [UserStack {
    data: [0; USER_STACK_SIZE],
}; MAX_APP_NUM];

pub fn init_app_ctx(app_id: usize) -> usize {
    KERNEL_STACKS[app_id].push_context(TrapContext::app_init_context(
        get_base_i(app_id),
        USER_STACKS[app_id].get_sp(),
    ))
}
