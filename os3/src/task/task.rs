//! Types related to task management

use super::TaskContext; 
use crate::config::{MAX_APP_NUM, MAX_SYSCALL_NUM};
#[derive(Copy, Clone)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub call_num:[u32;MAX_SYSCALL_NUM],
    pub call_time:usize,
    // LAB1: Add whatever you need about the Task.
}

#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}
