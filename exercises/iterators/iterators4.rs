// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.



pub fn factorial(num: u64) -> u64 {//阶乘
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
        (1..=num).product()


}
/*
使用范围表达式(1..=num)生成从1到num（包含num）的迭代器
调用.product()方法计算迭代器所有元素的乘积
空迭代器（当num=0时）的乘积自动返回1，符合数学定义
完全避免使用：
    return语句（通过表达式直接返回）
    命令式循环（使用迭代器方法）
    额外变量
    递归
    满足所有边界条件（包括0! = 1的特殊情况）
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
