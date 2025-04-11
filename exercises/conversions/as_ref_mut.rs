// as_ref_mut.rs
//
// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
//
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a
// hint.



// Obtain the number of bytes (not characters) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
//fn byte_counter<T>(arg: T) -> usize {
// Obtain the number of bytes (not characters) in the given argument.
fn byte_counter<T:AsRef<str>>(arg: T) -> usize {//要解决这些问题，我们需要正确使用AsRef和AsMut trait 来约束泛型参数。
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
// Obtain the number of characters (not bytes) in the given argument.
//fn char_counter<T>(arg: T) -> usize {
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using as_mut().
// TODO: Add the appropriate trait bound.
// Squares a number using as_mut().
//fn num_sq<T>(arg: &mut T) {
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    // TODO: Implement the function body.
    *arg.as_mut() *= *arg.as_mut();
}
/*
byte_counter 和 char_counter 的 Trait Bound
    将泛型参数 T 约束为 AsRef<str>，因为这两个函数需要处理字符串类型（&str 或 String）
    AsRef<str> 允许将 String 或 &str 转换为 &str，从而调用 as_bytes() 和 chars() 方法
num_sq 的 Trait Bound
    将泛型参数 T 约束为 AsMut<u32>，表示可以通过 as_mut() 获取内部 u32 的可变引用
    使用 *arg.as_mut() *= *arg.as_mut() 实现平方操作，直接修改底层值
代码逻辑解释：
    byte_counter：通过 AsRef<str> 获取字符串引用，然后转换为字节数组并计算长度
    char_counter：同样获取字符串引用，但直接计算字符数量
    num_sq：通过 AsMut<u32> 获取可变引用，直接修改原始值（例如 Box<u32> 中的值）
这些修改确保了类型安全，同时保持了代码的通用性，能够处理 String、&str 和 Box<u32> 等常见类型。
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
