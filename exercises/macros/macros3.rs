// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.


/*
在Rust中，宏默认只在定义它们的模块及其子模块中可见。
为了让宏在模块外部可用，需要使用#[macro_export]属性来导出宏。

解决方案如下：
    在宏定义前添加#[macro_export]属性，以导出宏。
    在调用宏之前，使用use语句将宏引入当前作用域。？？？？？存疑
    这样，宏既保持在模块内定义，又可以在模块外部调用。
*/
mod macros {
    #[macro_export] // 添加这个属性来导出宏
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

//use macros::my_macro; // 将宏引入当前作用域

fn main() {
    my_macro!();
}
