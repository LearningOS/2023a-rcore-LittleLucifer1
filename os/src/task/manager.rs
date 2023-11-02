//!Implementation of [`TaskManager`]
use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;

// const BIG_STRIDE: usize = 10000;
///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        // let mut minimal_stride = BIG_STRIDE;
        // for tcb in self.ready_queue.iter() {
        //     let stride = tcb.inner_exclusive_access().get_stride();
        //     if minimal_stride > stride {
        //         minimal_stride = stride;
        //     }
        // }
        // for _i in 0..self.ready_queue.len() {
        //     let result = self.ready_queue.pop_front();
        //     let result_inner = result.unwrap();
        //     if result_inner.inner_exclusive_access().get_stride() == minimal_stride {
        //         let new_stride = minimal_stride + BIG_STRIDE / result_inner.inner_exclusive_access().get_priority();
        //         result_inner.inner_exclusive_access().set_stride(new_stride);
        //         return Some(result_inner);
        //     }
        //     else {
        //         self.ready_queue.push_back(result_inner);
        //     }
        // }
        // None
        // self.ready_queue.pop_front()

         self.ready_queue.pop_front()

    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}
