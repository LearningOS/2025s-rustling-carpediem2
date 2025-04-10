// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.



/*
在 Rust 中，字符串切片（&str）的生命周期必须严格管理，以确保它们引用的数据在切片使用期间始终有效。
你的代码中存在一个生命周期问题，因为 longest 函数返回的切片可能引用了一个在内部块中定义的字符串，当内部块结束时，该字符串被销毁，导致悬垂引用（Dangling Reference）。
*/

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result;
    {
        //let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is '{}'", result);
}
/*
longest 函数：接受两个字符串切片 x 和 y，并使用生命周期注解 'a，表示 x 和 y 的生命周期必须至少为 'a。返回其中较长的一个切片。
main 函数：string1 是一个 String 类型的变量，其生命周期仅限于 main 函数的作用域。
    result 是一个变量，用于存储 longest 函数的返回值。
    在内部块中，定义了一个 String 类型的变量 string2，其生命周期仅限于该内部块。
    调用 longest(string1.as_str(), string2.as_str())，返回的切片可能引用 string1 或 string2。
潜在问题：
    如果 longest 返回的是 string2.as_str()，那么这个切片引用的是 string2 的数据。当内部块结束时，string2 被销毁，导致 result 中的切片成为悬垂引用。
*/
/*
调整 string2 的定义位置：将 string2 的定义从内部块移到 main 函数的外部块，使其生命周期与 result 一致。
    这样，string2 的生命周期将覆盖整个 main 函数的作用域，确保 longest 函数返回的切片在 println! 调用时仍然有效。
保持 longest 函数不变：longest 函数仍然接受两个字符串切片，并返回其中较长的一个。
    由于 string1 和 string2 现在都在外部块中定义，它们的生命周期足够长，可以确保返回的切片在 println! 调用时有效。
*/