// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
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
Rust的宏系统要求：
    宏必须在调用之前定义
    宏名称后需要加!符号调用（my_macro!()）
    宏可以包含任意合法的Rust代码
    宏通过模式匹配定义不同调用方式
*/