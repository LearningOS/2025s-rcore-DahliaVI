//! Process management syscalls
// use riscv::register::sie;

use crate::{
    task::{exit_current_and_run_next, get_syscall_count, suspend_current_and_run_next}, timer::get_time_us
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

// TODO: implement the syscall
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        0 => {
            let ptr = id as *const u8;
            let value = unsafe{*ptr};
            value as isize
        }
        1 => {
            let ptr = id as *mut u8;
            let data_byte = (data & 0xff) as u8;
            unsafe{*ptr = data_byte};
            return 0
        }
        2 => {
            // let current_task = current_task();
            // let task_inner = current_task.inner_exclusive_access();
            // let count = task_inner.syscall_counts.get(&id).copied().unwrap_or(0);
            // drop(task_inner);
            // count as isize
            let value = get_syscall_count(id);
            value as isize
        }
        _ => -1
    }
    // -1
}