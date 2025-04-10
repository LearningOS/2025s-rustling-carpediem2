// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.



trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.

//获取可变所有权：使用mut self获取向量的可变所有权,创建新字符串：显式创建String::from("Bar")
//推入向量：使用push()方法将新字符串添加到向量末尾
impl AppendBar for Vec<String>{
    fn append_bar(mut self) ->Self{
        self.push(String::from("Bar"));//在Rust中，向Vec添加元素通常使用push方法
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
