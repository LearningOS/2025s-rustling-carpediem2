// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.
//在上一个练习中，我们希望所有线程都完成它们的工作，但这次生成的线程需要负责更新共享值：JobStatus.jobs_completed


//use std::sync::Arc;
use std::sync::{Arc, atomic::{AtomicU32, Ordering}};
use std::thread;
use std::time::Duration;

struct JobStatus {
    //jobs_completed: u32,
    jobs_completed: AtomicU32, // 改为原子类型
}

fn main() {
    //let status = Arc::new(JobStatus { jobs_completed: 0 });
    let status = Arc::new(JobStatus { jobs_completed: AtomicU32::new(0) });//初始化原子值
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            // 使用原子操作更新值（无需显式锁）
            status_shared.jobs_completed.fetch_add(1, Ordering::Relaxed);
            //status_shared.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        //println!("jobs completed {}", ???);
        println!("jobs completed {}", status.jobs_completed.load(Ordering::Relaxed));
    
    }
}

/*
将字段类型改为AtomicU32（需要修改结构体定义，但这是实现线程安全的必要修改）
使用原子操作fetch_add替代直接赋值
使用load方法原子读取值
保持原有代码结构，但通过原子操作保证线程安全
*/