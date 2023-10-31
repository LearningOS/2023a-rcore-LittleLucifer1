//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, 
        suspend_current_and_run_next, 
        TaskStatus, current_user_token,
        get_first_run_time, get_task_syscall_time,
        alloc_memory_to_memset, translate_vpn_ppn,
        unmap_framed_area,
    },
    mm::{virt_to_phys, VirtAddr, MapPermission, VirtPageNum}, 
    timer::{get_time_us, get_time_ms},
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let virt_addr = VirtAddr::from(_ts as usize);
    if let Some(phys_addr) = virt_to_phys(virt_addr, current_user_token()) {
        let us = get_time_us();
        let ts = phys_addr.0 as *mut TimeVal;
        unsafe{
            *ts = TimeVal {
                sec: us / 1_000_000,
                usec: us % 1_000_000,
            };
        }
        0
    }
    else {
        -1
    }   
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    let virt_addr = VirtAddr::from(_ti as usize);
    if let Some(phys_addr) = virt_to_phys(virt_addr, current_user_token()) {
        let ti = phys_addr.0 as *mut TaskInfo;
        unsafe {
            (*ti).status = TaskStatus::Running;
            (*ti).time = get_time_ms() - get_first_run_time();
            (*ti).syscall_times = get_task_syscall_time();
        }
        0
    }
    else {
        -1
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    let virt_addr = VirtAddr::from(_start);
    if !virt_addr.aligned() {
        -1
    }
    else if _port > 7usize || _port == 0 {
        -1
    }
    else {
        let start_vpn = VirtPageNum::from(VirtAddr::from(_start));
        let end_vpn = VirtPageNum::from(VirtAddr::from(_start + _len).ceil());

        for i in start_vpn.0..end_vpn.0 {
            if let Some(pte) = translate_vpn_ppn(VirtPageNum::from(i)) {
                if pte.is_valid() {
                    return -1;
                }
            }
        }

        let mut permission = MapPermission::U;
        if (_port & 0x1) == 0x1 {
            permission |= MapPermission::R;
        }
        if (_port & 0x2) == 0x2 {
            permission |= MapPermission::W;
        }
        if (_port & 0x4) == 0x4 {
            permission |= MapPermission::X;
        }
        
        alloc_memory_to_memset(VirtAddr(_start), VirtAddr(_start + _len), permission);
        0
    }
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    let virt_addr = VirtAddr::from(_start);
    if !virt_addr.aligned() {
        return -1;
    }
    let start_vpn = VirtPageNum::from(VirtAddr::from(_start));
    let end_vpn = VirtPageNum::from(VirtAddr::from(_start + _len).ceil());

    unmap_framed_area(start_vpn, end_vpn)

}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
