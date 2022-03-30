#[derive(Copy, Clone)]
#[repr(C)]
pub struct TaskContext {
    /// return address register
    ra: usize,
    /// stack pointer register
    sp: usize,
    /// `[s0~s11]` Saved registers
    s: [usize; 12],
}

impl TaskContext {
    pub fn init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }

    // pub fn goto_restore(kernel_stack_ptr: usize) -> Self {
    //     extern "C" {
    //         fn __restore();
    //     }
    //
    //     Self {
    //         ra: __restore as usize,
    //         sp: kernel_stack_ptr,
    //         s: [0; 12],
    //     }
    // }

    pub fn goto_restore(mut self, kernel_stack_ptr: usize) -> Self {
        extern "C" {
            fn __restore();
        }

        self.ra = __restore as usize;
        self.sp = kernel_stack_ptr;

        self
    }
}
