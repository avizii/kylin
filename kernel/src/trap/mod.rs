use crate::batch::run_next_app;
use crate::syscall::syscall;
use crate::{info, println};
use core::arch::global_asm;
use riscv::register::mtvec::TrapMode;
use riscv::register::scause::Exception;
use riscv::register::scause::Trap;
use riscv::register::{scause, sepc, stval, stvec};
use stvec::write;

mod context;

pub use context::TrapContext;

global_asm!(include_str!("trap.S"));

// initialize CSR `stvec` as the entry of `__alltraps`
pub fn init() {
    // FFI
    extern "C" {
        fn __alltraps();
    }

    unsafe {
        // stvec -> 控制 Trap 处理代码的入口地址
        write(__alltraps as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    let sepc = sepc::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            info!("[kernel] spec: {:#x}", sepc);
            println!("[kernel] Receive User Environment Call, kernel handle it.");
            cx.sepc += 4; // trap处理后的下一条指令地址 这里下一条指令是 __restore 的入口
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            run_next_app();
        }
        _ => panic!(
            "Unsupported trap {:?}, stval = {:#x}!",
            scause.cause(),
            stval
        ),
    }
    cx
}
