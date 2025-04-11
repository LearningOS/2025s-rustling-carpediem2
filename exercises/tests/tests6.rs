// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.



struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: The `ptr` contains an owned box of `Foo` by contract. We
    // simply reconstruct the box from that pointer.
    //let mut ret: Box<Foo> = unsafe { ??? };
    //todo!("The rest of the code goes here")
    // SAFETY: Reconstruct the box from the raw pointer, which we own
    let mut ret: Box<Foo> = unsafe { Box::from_raw(ptr) };
    
    // Modify the Option<String> field to meet test requirement
    ret.b = Some("hello".to_owned());
    
    ret
}
/*
Box重构：
    使用Box::from_raw(ptr)将原始指针转换回Box
    这是Rust标准库中用于从原始指针重建Box的标准方法
字段修改：
    显式设置ret.b = Some("hello".to_owned())
    满足测试用例中ret.b需要为Some的要求
安全契约：
    在文档注释中明确说明函数会修改b字段
    强调所有权转移：原始指针被函数接管后，调用者不应再访问原始指针
*/
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;
        // SAFETY: We pass an owned box of `Foo`.
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &ret.a as *const u128 as usize;

        assert!(ptr_1 == ptr_2);
        assert!(ret.b == Some("hello".to_owned()));
    }
}
