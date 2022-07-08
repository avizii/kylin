use core::arch::asm;

pub fn sfence_vma() {
    unsafe {
        // sfence.vma zero, zero
        asm!("sfence.vma");
    }
}
