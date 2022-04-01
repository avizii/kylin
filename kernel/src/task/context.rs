#[derive(Copy, Clone)]
#[repr(C)]
pub struct TaskContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl TaskContext {
    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }
    pub fn goto_restore(kstack_ptr: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}

use core::fmt::{Display, Formatter};

impl Display for TaskContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "ra: {:#x}, sp: {:#x}, s0~s11: [{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}]",
            self.ra,
            self.sp,
            self.s[0],
            self.s[1],
            self.s[2],
            self.s[3],
            self.s[4],
            self.s[5],
            self.s[6],
            self.s[7],
            self.s[8],
            self.s[9],
            self.s[10],
            self.s[11]
        )
    }
}
