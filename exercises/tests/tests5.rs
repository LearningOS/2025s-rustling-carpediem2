// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
//
// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
//
// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.



/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
/// /// `address` 必须是一个指向可变 `u32` 值的有效指针。调用者必须确保：
/// 1. 指针已正确对齐
/// 2. 在整个操作期间，指向的内存保持有效
/// 3. 不存在会导致数据竞争的其他别名
unsafe fn modify_by_address(address: usize) {
    // TODO: Fill your safety notice of the code block below to match your
    // code's behavior and the contract of this function. You may use the
    // comment of the test below as your format reference.

    unsafe {
        //todo!("Your code goes here")
        *(address as *mut u32) = 0xAABBCCDD;
    }
}
/*
实现逻辑：
将 usize 地址转换回 *mut u32 指针
执行原始指针解引用并赋值
使用测试用例中指定的魔数值（0xAABBCCDD）

*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
