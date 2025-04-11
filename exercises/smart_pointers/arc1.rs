// arc1.rs
//
// In this exercise, we are given a Vec of u32 called "numbers" with values
// ranging from 0 to 99 -- [ 0, 1, 2, ..., 98, 99 ] We would like to use this
// set of numbers within 8 different threads simultaneously. Each thread is
// going to get the sum of every eighth value, with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...
//
// Because we are using threads, our values need to be thread-safe.  Therefore,
// we are using Arc.  We need to make a change in each of the two TODOs.
//
// Make this code compile by filling in a value for `shared_numbers` where the
// first TODO comment is, and create an initial binding for `child_numbers`
// where the second TODO comment is. Try not to create any copies of the
// `numbers` Vec!
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.
/*
在本练习中，我们得到一个名为 “numbers” 的 u32 Vec，其值范围为 0 到 99 -- [ 0， 1， 2， ...， 98， 99 ] 我们想在 8 个不同的线程中同时使用这组数字。每个线程都将获得每 8 个值的总和，并带有偏移量。
//
第一个线程（偏移量 0）将求和为 0、8、16、...
第二个线程（偏移量 1）将求和为 1、9、17、...
第三个线程（偏移量 2）将求和为 2、10、18、...
// ...
第八个线程（偏移量 7）将相加 7、15、23、...
//
因为我们使用的是线程，所以我们的值需要是线程安全的。 因此，我们使用的是 Arc。 我们需要在两个 TODO 中分别进行更改。
//
通过在第一个 TODO 注释所在的 'shared_numbers' 中填写一个值来编译此代码，并在第二个 TODO 注释所在的位置为 'child_numbers' 创建一个初始绑定。尽量不要创建任何 'numbers' Vec 的副本！
*/


/*
题目要求使用Arc来确保线程安全，所以原来的numbers需要被包装成Arc。然后每个线程中的child_numbers应该是一个指向这个Arc的克隆，这样多个线程可以共享同一个数据，而不用复制整个Vec。
这样shared_numbers的类型就是Arc<Vec<u32>>。

然后第二个TODO是child_numbers的初始绑定。在循环中，每个线程需要访问这个Arc，所以应该克隆Arc的引用。因为Arc实现了Clone，克隆的时候会增加引用计数，这样每个线程都有自己的Arc实例，指向同一个数据。

在循环内部，每次循环都会创建一个新的线程，而child_numbers需要被移动到闭包中。因为闭包是move的，所以需要确保child_numbers的所有权被转移。所以，在循环体内部，每次迭代都应该克隆shared_numbers，然后将这个克隆后的Arc赋值给child_numbers，这样每个线程都有自己的Arc实例，可以安全地访问数据。

需要注意，在循环内部，每次迭代都会创建一个新的child_numbers，即每次克隆shared_numbers。这样每个线程的child_numbers都是Arc的克隆，这样是正确的，因为Arc的clone是浅拷贝，增加引用计数，而不会复制数据。

然后，在闭包内部，child_numbers.iter()会遍历整个Vec，但每个线程只处理符合自己偏移量的元素。比如，offset是0的线程，过滤出所有n%8==0的元素，然后求和。

这样，整个代码应该能正确编译，并且每个线程都能正确访问到共享的numbers数据，而无需复制整个Vec。

child_numbers = Arc::clone(&shared_numbers); 或者直接shared_numbers.clone()，因为Arc的clone方法等同于Arc::clone(&self)。
*/
#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);// TODO // 使用Arc包装原始Vec实现线程共享
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        //let child_numbers = Arc::clone(&shared_numbers);// TODO // 克隆Arc指针而非数据副本
        let child_numbers = shared_numbers.clone();
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
/*
shared_numbers 使用 Arc::new(numbers) 将原始Vec包装成线程安全的智能指针
child_numbers 通过 Arc::clone(&shared_numbers) 创建引用计数指针的浅拷贝，所有线程共享同一份底层数据
每个线程通过 move 闭包获取独立的Arc所有权，确保线程安全访问共享数据
整个过程没有复制原始Vec，仅通过引用计数管理共享访问
*/