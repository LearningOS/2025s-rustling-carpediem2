// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.



#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    }; // 这里添加了分号
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }; // 这里添加了分号
}
/*
在Rust中，macro_rules!定义宏时，每个匹配分支需要用分号;结束。
保持了宏的两个重载版本：
    无参版本 my_macro!()
    带表达式参数版本 my_macro!(7777)
*/
fn main() {
    my_macro!();
    my_macro!(7777);
}
