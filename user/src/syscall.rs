use core::arch::asm;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;

///
///
/// # Arguments
///
/// * `id`: 系统调用ID
/// * `args`: 系统调用参数
///
/// returns: isize
///
/// # Examples
///
/// ```
///
/// ```
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret, // x10 作为输入，又作为输出
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}

/// 将内存中缓冲区的数据写入文件
///
/// # Arguments
///
/// * `fd`: 表示待写入文件的文件描述符
/// * `buffer`: 表示内存缓冲区
///
/// returns: isize 成功写入的长度
///
/// # Examples
///
/// ```
///
/// ```
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

/// 退出应用程序并将返回值告知批处理系统
///
/// # Arguments
///
/// * `exist_code`: 表示应用程序的返回值
///
/// returns: isize
///
/// # Examples
///
/// ```
///
/// ```
pub fn sys_exit(exist_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exist_code as usize, 0, 0])
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}