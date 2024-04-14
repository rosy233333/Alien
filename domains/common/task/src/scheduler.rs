use alloc::sync::Arc;
use core::hint::spin_loop;

use crate::{
    processor::{add_task, current_cpu, pick_next_task, take_current_task},
    task::{Task, TaskStatus},
};

pub fn run_task() -> ! {
    loop {
        if let Some(task) = pick_next_task() {
            // update state to running
            task.update_state(TaskStatus::Running);
            // get the process context
            let context = task.get_context_raw_ptr();
            let tid = task.tid();
            let cpu = current_cpu();
            // basic::println!("switch to task: {:?}", task.pid());
            cpu.set_current(task);
            // switch to the process context
            let cpu_context = cpu.get_idle_task_cx_ptr();
            // println!("switch to task: {:?}", tid);
            basic::task::switch(cpu_context, context, tid);
        } else {
            spin_loop();
        }
    }
}

pub fn schedule_now(task: Arc<Task>) {
    let context = task.get_context_mut_raw_ptr();
    match task.status() {
        TaskStatus::Waiting => {
            drop(task);
        }
        TaskStatus::Zombie => {
            // 退出时向父进程发送信号，其中选项可被 sys_clone 控制
            // if task.send_sigchld_when_exit || task.pid == task.tid.0 {
            //     let parent = task
            //         .access_inner()
            //         .parent
            //         .as_ref()
            //         .unwrap()
            //         .upgrade()
            //         .unwrap();
            //     // send_signal(parent.pid, SignalNumber::SIGCHLD as usize);
            // }
            // task.terminate(); // release some resources
        }
        _ => {
            // println!("add task to scheduler");
            add_task(task);
        }
    }
    let cpu = current_cpu();
    let cpu_context = cpu.get_idle_task_cx_ptr();
    basic::task::switch(context, cpu_context, usize::MAX);
}

pub fn do_suspend() {
    let task = take_current_task().unwrap();
    // task.access_inner().update_timer();
    // check_task_timer_expired();
    task.update_state(TaskStatus::Ready);
    schedule_now(task);
}