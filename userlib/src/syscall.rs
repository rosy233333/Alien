use core::arch::asm;
use crate::{syscall, syscall_id};
syscall_id!(SYSCALL_READ,63);
syscall_id!(SYSCALL_WRITE,64);
syscall_id!(SYSCALL_EXIT,93);
syscall_id!(SYSCALL_YIELD,124);
syscall_id!(SYSCALL_GET_TIME,169);
syscall_id!(SYSCALL_GETPID,172);
syscall_id!(SYSCALL_FORK,220);
syscall_id!(SYSCALL_EXEC,221);
syscall_id!(SYSCALL_WAITPID,260);


fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
        "ecall",
        inlateout("x10") args[0] => ret,
        in("x11") args[1],
        in("x12") args[2],
        in("x17") id
        );
    }
    ret
}

syscall!(sys_read,SYSCALL_READ,usize,*mut u8,usize);
syscall!(sys_write,SYSCALL_WRITE,usize,*const u8,usize);
syscall!(sys_exit,SYSCALL_EXIT,i32);
syscall!(sys_yield,SYSCALL_YIELD);
syscall!(sys_getpid,SYSCALL_GETPID);
syscall!(sys_get_time,SYSCALL_GET_TIME);
syscall!(sys_fork,SYSCALL_FORK);
syscall!(sys_execve,SYSCALL_EXEC,*const u8);
syscall!(sys_waitpid,SYSCALL_WAITPID,usize,*mut i32);