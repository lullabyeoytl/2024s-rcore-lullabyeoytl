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
    /// The start time of the task
    pub start_time:usize,  
    ///first this task is scsheduled
    pub task_info: TaskInfo,
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

#[allow(dead_code)]
#[derive(Copy, Clone)]
/// The information of a task
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

impl TaskInfo {
    /// Create a new TaskInfo with given status
    pub fn new(status: TaskStatus) -> Self {
        let syscall_times = [0;MAX_SYSCALL_NUM];
        TaskInfo { status, syscall_times, time:0, }
    }
    /// Add syscall time to the task
    pub fn add_syscall_time(&mut self, index: usize){
        if index < MAX_SYSCALL_NUM {
            self.syscall_times[index] += 1;
        }
    }
   /// Set the status of the task   
    pub fn set_status(&mut self, status: TaskStatus){
        self.status = status;
    }
    /// Set the running time of the task   
    pub fn set_time(&mut self, time: usize){
        self.time = time;
    }
}