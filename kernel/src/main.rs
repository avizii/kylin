#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;
// use sbi::sleep;

mod console;
mod lang_items;
mod log;
mod sbi;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
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
    }
    clear_bss();
    println!("Hello, AviziiOS!");
    // sleep(3_000_000); invalid

    info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    error!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    warn!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as usize, boot_stack_top as usize
    );
    trace!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);

    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    ((sbss as usize)..(ebss as usize)).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}
