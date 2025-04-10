// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.



struct Book<'a> {
    author: &'a str,
    title: &'a str,
}
/*
Book<'a> 是一个泛型结构体，它带有一个生命周期参数 'a。
    author 和 title 字段的类型是 &'a str，表示它们都是字符串切片，并且这些切片引用的数据的生命周期至少为 'a。
生命周期注解的含义：
    'a 生命周期参数确保 author 和 title 字段引用的字符串切片在 Book<'a> 结构体实例的生命周期内有效。
    这意味着，只要 Book<'a> 结构体实例存在，author 和 title 字段引用的字符串数据就必须存在。
*/
fn main() {
    let name = String::from("Jill Smith");//name 和 title 是 String 类型的变量，它们的生命周期仅限于 main 函数的作用域。
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };//创建 Book<'a> 结构体实例，'a 生命周期参数与 name 和 title 的生命周期一致
    //由于 Book<'a> 结构体带有生命周期注解 'a，编译器会确保 author 和 title 字段引用的数据在 book 实例的生命周期内有效

    println!("{} by {}", book.title, book.author);
}
