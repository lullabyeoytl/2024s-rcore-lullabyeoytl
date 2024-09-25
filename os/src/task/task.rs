//! Types related to task management

use crate::config::MAX_SYSCALL_NUM;

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    // @lullabyeoytl
    // the number of times each system call is called
    pub syscall_times:[u32;MAX_SYSCALL_NUM],
    // the total number of system calls
    pub syscall_count:u32,
    // the time when task is created
    pub start_time:usize,  
     //first this task is scsheduled
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
