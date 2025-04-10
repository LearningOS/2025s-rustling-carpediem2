// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: the value of the current item and
// the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `todo!()`
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.



/*
在编译时，Rust 需要知道一个类型占用了多少空间。这对于递归类型来说就成问题了，其中值可以具有另一个相同类型的值作为自身的一部分。为了解决这个问题，我们可以使用 'Box' - 一个用于在堆上存储数据的智能指针，它还允许我们包装递归类型。
//
我们在本练习中实现的递归类型是 'cons list' - 函数式编程语言中常见的一种数据结构。cons 列表中的每个项目都包含两个元素：当前项目的值和下一个项目的值。最后一项是名为 'Nil' 的值。
//
第 1 步：在枚举定义中使用 'Box' 来编译代码
第 2 步：通过替换 'todo！()`
*/
#[derive(PartialEq, Debug)]
pub enum List {
    //Cons(i32, List),
    Cons(i32, Box<List>), // 使用Box包装递归类型
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    //todo!()
    List::Nil // 直接返回Nil枚举值
}
/*
递归类型处理：
    将Cons变体的第二个字段从直接List改为Box<List>。Box智能指针将递归类型存储在堆上，使编译器能确定类型大小（指针固定大小），从而解决递归类型大小未知的编译错误。
列表构造：
    create_empty_list直接返回List::Nil枚举值，表示空列表。
    create_non_empty_list构造一个包含单个元素1的列表，使用Box::new(List::Nil)包装递归的Nil，确保类型系统正确。
*/
pub fn create_non_empty_list() -> List {
    //todo!()
    // 使用Box包装递归的Nil，创建包含单个元素的列表
    List::Cons(1, Box::new(List::Nil))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
