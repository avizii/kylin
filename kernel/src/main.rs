#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

#[macro_use]
mod batch;
mod config;
mod console;
mod lang_items;
mod loader;
mod log;
mod sbi;
mod stack;
mod stack_trace;
mod sync;
mod syscall;
mod task;
mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    /*    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }*/
    clear_bss();
    println!("[kernel] Hello, Kylin!");
    trap::init();
    batch::init();
    batch::run_next_app();
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    ((sbss as usize)..(ebss as usize)).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}
