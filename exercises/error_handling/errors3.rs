// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.


//使用 if let 语法：if let Ok(cost) = total_cost(pretend_user_input) 这一行直接提取了 Ok 分支的值。
//如果 total_cost 返回 Ok，则执行大括号内的代码块。如果 total_cost 返回 Err，则跳过 if let 块，执行 else 块中的错误处理代码。
//简化错误处理：在 else 块中，直接输出一个通用的错误信息，表明输入无法解析。
use std::num::ParseIntError;

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    if let Ok(cost) = total_cost(pretend_user_input){
        if cost > tokens {
            println!("You can't afford that many!");
        } 
        else {
            tokens -= cost;
            println!("You now have {} tokens.", tokens);
        }
    }else {
            // 处理 Err 分支
            println!("Invalid input: Could not parse quantity.");
        
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
