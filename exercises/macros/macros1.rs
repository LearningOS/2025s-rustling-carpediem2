// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.



macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
/*
macro_rules! my_macro 定义了一个名为my_macro的宏
() => { ... }; 表示当宏以空参数形式调用时（即my_macro!()），执行大括号内的代码
在宏体内使用println!宏输出消息
main函数中通过my_macro!();调用该宏
*/