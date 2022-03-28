use riscv::register::sstatus;
use riscv::register::sstatus::{Sstatus, SPP};

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus, // SPP 等字段给出 Trap 发生之前 CPU 处在哪个特权级（S/U）等信息
    pub sepc: usize, // 当 Trap 是一个异常的时候，记录 Trap 发生之前执行的最后一条指令的地址  会被修改为 Trap 处理完成后默认会执行的下一条指令的地址
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);

        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
        };

        cx.set_sp(sp);

        cx
    }
}
