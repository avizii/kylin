#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

use core::arch::global_asm;

extern crate alloc;
extern crate bitflags;

#[macro_use]
mod config;
mod console;
mod lang_items;
mod loader;
mod log;
mod mm;
mod riscv;
mod sbi;
mod stack_trace;
mod sync;
mod syscall;
mod task;
mod timer;
mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[kernel] Hello, Kylin!");
    trap::init();
    loader::load_apps();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    println!("[kernel] Start to run applications!");
    task::run_first_task();
    panic!("Unreachable in rust_main!")
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    ((sbss as usize)..(ebss as usize)).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}
