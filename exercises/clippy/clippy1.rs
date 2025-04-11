// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.



use std::f32;

fn main() {

    //let pi = 3.14f32;
    // 使用标准库提供的精确π常量
    //f32::consts::PI是Rust标准库提供的f32类型π常量（值约为3.1415927）
    let pi = f32::consts::PI;
    let radius = 5.00f32;
//f32::powi用于整数指数，但你的半径是f32类型，应该使用f32::powf（用于浮点数指数）
    //let area = pi * f32::powi(radius, 2);
    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
