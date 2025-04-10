// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.



//struct Wrapper {
//    value: u32,
//}

struct Wrapper<T> {//泛型结构体：Wrapper<T>现在可以存储任何类型T
    value: T,
}

//impl Wrapper {
//    pub fn new(value: u32) -> Self {
//        Wrapper { value }
//    }
//}

impl<T> Wrapper<T> {//泛型实现：impl<T> Wrapper<T>为所有可能的类型提供实现

    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }


}
