// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!



fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");//&str
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}


//string_slice("blue"): "blue" 是一个字符串字面量，类型为 &str，因此调用 string_slice。
//string("red".to_string()): "red".to_string() 将 &str 转换为 String，因此调用 string。
//string(String::from("hi")): String::from("hi") 创建一个 String，因此调用 string。
//string("rust is fun!".to_owned()): "rust is fun!".to_owned() 将 &str 转换为 String，因此调用 string。
//string("nice weather".into()): "nice weather".into() 将 &str 转换为 String（因为实现了 From<&str> for String），因此调用 string。
// string(format!("Interpolation {}", "Station")): format! 宏返回一个 String，因此调用 string。
//string_slice(&String::from("abc")[0..1]): &String::from("abc")[0..1] 是一个字符串切片，类型为 &str，因此调用 string_slice。
//string_slice(" hello there ".trim()): trim() 方法返回一个 &str，因此调用 string_slice。
//string("Happy Monday!".to_string().replace("Mon", "Tues")): replace() 方法返回一个 String，因此调用 string。
//string("mY sHiFt KeY iS sTiCkY".to_lowercase()): to_lowercase() 方法返回一个 String，因此调用 string。