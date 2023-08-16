//! Futex按英文翻译过来就是快速用户空间互斥体(Fast Userspace Mutex)。在传统的Unix系统中，System V IPC(inter process communication)，
//! 如 semaphores, msgqueues, sockets还有文件锁机制(flock())等进程间同步机制都是对一个内核对象操作来完成的，这
//! 个内核对象对要同步的进程都是可见的，其提供了共享的状态信息和原子操作。当进程间要同步的时候必须要通过系统调用
//! (如semop())在内核中完成。可是经研究发现，很多同步是无竞争的，即某个进程进入互斥区，到再从某个互斥区出来这段
//! 时间，常常是没有进程也要进这个互斥区或者请求同一同步变量的。但是在这种情况下，这个进程也要陷入内核去查询是否发生竞争
//! 退出的时侯还要陷入内核去查询是否有进程等待在同一同步变量上。这些不必要的系统调用(或者说内核陷入)造
//! 成了大量的性能开销。为了解决这个问题，设计了Futex这一结构。Futex是一种用户态和内核态混合的同步机制。首先，同步的进
//! 程间通过mmap共享一段内存，futex变量就位于这段共享的内存中且操作是原子的，当进程尝试进入互斥区或者退出互斥区的
//! 时候，先去查看共享内存中的futex变量，如果没有竞争发生，则只修改futex, 而不用再执行系统调用了。当通过访问futex
//! 变量告诉进程有竞争发生，则还是得执行系统调用去完成相应的处理(wait 或者 wake up)。简单的说，futex就是通过在用户
//! 态的检查，（motivation）如果了解到没有竞争就不用陷入内核了，大大提高了low-contention时候的效率。Linux从2.5.7开始支持Futex。
//! 
//! Reference: https://cloud.tencent.com/developer/article/1176832
//! 
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::min;

use kernel_sync::Mutex;

use crate::error::{AlienError, AlienResult};
use crate::task::{Task, TASK_MANAGER};
use crate::timer::read_timer;

pub struct FutexWaiter {
    task: Option<Arc<Task>>,
    wait_time: Option<usize>,
    timeout_flag: Arc<Mutex<bool>>,
}

pub struct FutexWaitManager {
    map: BTreeMap<usize, Vec<FutexWaiter>>,
}

impl FutexWaiter {
    pub fn new(task: Arc<Task>, wait_time: Option<usize>, timeout_flag: Arc<Mutex<bool>>) -> Self {
        Self {
            task: Some(task),
            wait_time,
            timeout_flag,
        }
    }

    pub fn wake(&mut self) -> Arc<Task> {
        self.task.take().unwrap()
    }
}

impl FutexWaitManager {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    /// 在某等待队列中加入等待进程
    pub fn add_waiter(&mut self, futex: usize, waiter: FutexWaiter) {
        self.map.entry(futex).or_insert(Vec::new()).push(waiter);
    }

    ///由于信号引发的唤醒操作
    pub fn wake_for_signal(&mut self) {
        for (_, waiters) in self.map.iter_mut() {
            let mut record = vec![];
            for (index, waiter) in waiters.iter_mut().enumerate() {
                let task = waiter.task.as_ref().unwrap();
                let task_inner = task.access_inner();
                let receiver = task_inner.signal_receivers.lock();
                if receiver.have_signal() {
                    drop(receiver);
                    drop(task_inner);
                    let mut task_manager = TASK_MANAGER.lock();
                    let task = waiter.wake();
                    task_manager.push_back(task);
                    record.push(index);
                }
            }
            record.iter().for_each(|index| {
                waiters.remove(*index);
            })
        }
        self.delete_empty_waiters();
    }

    /// 由于超时引发的唤醒操作
    pub fn wake_for_timeout(&mut self) {
        let now = read_timer();
        for (_, waiters) in self.map.iter_mut() {
            let mut record = vec![];
            for (index, waiter) in waiters.iter_mut().enumerate() {
                if let Some(wait_time) = waiter.wait_time {
                    if wait_time <= now {
                        *waiter.timeout_flag.lock() = true;
                        let task = waiter.wake();
                        let mut task_manager = TASK_MANAGER.lock();
                        task_manager.push_back(task);
                        record.push(index);
                    }
                }
            }
            record.iter().for_each(|index| {
                waiters.remove(*index);
            })
        }
        // delete empty waiters
        self.delete_empty_waiters();
    }

    /// 清空所有空的等待队列
    fn delete_empty_waiters(&mut self) {
        let mut record = vec![];
        for (futex, waiters) in self.map.iter() {
            if waiters.is_empty() {
                record.push(*futex);
            }
        }
        record.iter().for_each(|futex| {
            self.map.remove(futex);
        })
    }

    /// 唤醒 futex 上的至多 num 个等待的进程
    pub fn wake(&mut self, futex: usize, num: usize) -> AlienResult<usize> {
        if let Some(waiters) = self.map.get_mut(&futex) {
            error!("there are {} waiters, wake {}", waiters.len(), num);
            let min_index = min(num, waiters.len());
            for i in 0..min_index {
                let task = waiters[i].wake();
                let mut task_manager = TASK_MANAGER.lock();
                task_manager.push_back(task);
            }
            // delete waiters
            waiters.drain(0..min_index);
            warn!("wake {} tasks", min_index);
            Ok(min_index)
        } else {
            error!("futex {} not found", futex);
            Err(AlienError::Other)
        }
    }

    /// 将原来等待在 old_futex 上至多 num 个进程转移到 requeue_futex 上等待，返回转移的进程数
    pub fn requeue(
        &mut self,
        requeue_futex: usize,
        num: usize,
        old_futex: usize,
    ) -> AlienResult<usize> {
        if num == 0 {
            return Ok(0);
        }
        // move waiters
        let mut waiters = self.map.remove(&old_futex).unwrap();
        // create new waiters
        let new_waiters = self.map.entry(requeue_futex).or_insert(Vec::new());
        let min_index = min(num, waiters.len());
        error!("requeue {} waiters", min_index);
        for _ in 0..min_index {
            let waiter = waiters.pop().unwrap();
            new_waiters.push(waiter);
        }
        // insert old waiters
        if !waiters.is_empty() {
            self.map.insert(old_futex, waiters);
        }
        Ok(min_index)
    }
}
