// using_as.rs
//
// Type casting in Rust is done via the usage of the `as` operator. Please note
// that the `as` operator is not only used when type casting. It also helps with
// renaming imports.
//
// The goal is to make sure that the division does not fail to compile and
// returns the proper type.
//
// Execute `rustlings hint using_as` or use the `hint` watch subcommand for a
// hint.



fn average(values: &[f64]) -> f64 {
    //let total = values.iter().sum::<f64>();
    //total / values.len() // 这里values.len()返回usize类型，不能直接与f64相除

    let total: f64 = values.iter().sum(); // 显式指定sum的类型
    total / values.len() as f64 // 使用as运算符进行类型转换
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}

/*
在average函数中，values.len()返回的是usize类型，而total是f64类型。在Rust中，不同类型的变量不能直接进行运算，因此需要将usize转换为f64。

为了解决这个问题，可以使用Rust的as运算符将usize转换为f64。as运算符在Rust中用于显式类型转换，可以将一种类型的值转换为另一种类型，前提是这种转换是安全的

values.iter().sum::<f64>() → values.iter().sum()：
    虽然sum()默认会返回与迭代器元素相同的类型（这里是f64），但显式指定类型参数可以：
        提高代码可读性
        在某些复杂情况下帮助类型推断
values.len() as f64：
    values.len()返回usize类型（无符号整数）
    使用as运算符将其显式转换为f64类型
    这使得除法操作在f64类型上进行，避免整数除法的问题
类型安全：
    Rust是强类型语言，不同数值类型不能直接运算
    as运算符提供了显式的类型转换机制
    这种转换需要保证目标类型可以安全表示源类型的值（例如将小范围的整数转换为大范围的浮点数总是安全的）
*/