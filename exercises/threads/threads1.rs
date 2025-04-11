// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.

/*
此程序会生成多个线程，每个线程至少运行 250 毫秒，每个线程返回完成这些线程所花费的时间。程序应等待所有生成的线程都已完成，并应将其返回值收集到一个 vector 中。
*/


use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        results.push(handle.join().unwrap()) // 使用join()获取线程返回值
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}

/*
主函数里创建了一个handles向量，里面保存了每个线程的句柄。每个线程执行一个闭包，闭包内部睡眠250毫秒，然后返回经过的时间。

这里应该使用handle.join()来获取每个线程的返回值，并将结果存入results向量中。

所以，在循环遍历handles的时候，应该对每个handle调用join()方法。join()会阻塞当前线程，直到对应的子线程结束，并返回Result<T>，其中T是闭包返回的类型。在这里，闭包返回的是u128，所以每个handle.join()会得到Result<u128>。需要将这些结果收集到results向量中。

使用handle.join().unwrap()获取线程返回值：
    join()会阻塞主线程直到子线程完成
    unwrap()用于处理可能的线程panic（根据题意假设线程不会panic）
将获取到的u128类型耗时结果存入results向量
最终通过遍历results打印每个线程的实际执行时间
这个程序会：
    创建10个线程，每个线程休眠250ms
    主线程等待所有子线程完成
    收集并打印每个线程的实际执行时间（可能略大于250ms，包含线程调度开销）
*/