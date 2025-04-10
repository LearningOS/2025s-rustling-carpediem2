// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.



trait AppendBar {
    fn append_bar(self) -> Self;
}

/*
fn append_bar(self) -> Self {  
    self + "Bar"  
}

fn append_bar(mut self) -> Self {  
    self.push_str("Bar");  
    self  
}
这两种方法都可以，但使用push_str可能更高效，因为它直接修改字符串内容，而不是创建新的字符串。
不过，由于append_bar方法接受self的所有权，我们可以选择消耗原来的字符串，并返回修改后的版本。
*/
impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
    fn append_bar(mut self) -> Self{//使用mut self获取字符串的可变所有权,通过获取所有权确保独占访问
        self.push_str("Bar");//通过push_str()方法直接修改字符串内容,直接在原字符串缓冲区追加内容
        self //返回Self类型允许连续调用方法
    }
    //当调用s.append_bar()时，原字符串会被修改，并返回新的字符串。测试用例中的断言就可以正确通过。
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
