// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut res = 42;
    let option = Some(12);
    //for x in option {
    if let Some(x) = option {//if let专门用于处理Option和Result等可匹配类型 比while let更简洁，因为只需要处理一次
        res += x;
    }
    println!("{}", res);
}
/*
在Rust中，for循环通常用于迭代集合或迭代器，而Option并不实现IntoIterator特性，因此不能直接使用for循环。
正确的做法是使用if let来检查Option是否包含值，并在包含值的情况下执行相应的代码。
Option是一个枚举类型，只能包含一个值或为空，因此不能直接迭代。

对Option使用for循环：
    Rust不允许直接对Option使用for循环，因为Option不是可迭代类型
未处理None情况：
    当option为None时，for循环不会执行任何操作，但编译器仍会发出警告

将for x in option改为if let Some(x) = option
    if let专门用于处理Option和Result等可匹配类型
比while let更简洁，因为只需要处理一次
    添加了处理None的隐含逻辑（当option为None时，不会执行任何操作）
*/