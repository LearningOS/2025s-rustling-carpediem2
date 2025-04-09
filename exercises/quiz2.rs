// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!



pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![]; //let mut output = Vec::new();
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            let transformered = match command{
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command:: Append(n)=>{
                    let append_str="bar".repeat(*n);//error[E0308]: mismatched types 问题出在.repeat(n)这里，repeat方法需要的是usize类型的参数，但你传入了&usize（引用类型）。
                    //修复方法：在n前加星号解引用 .repeat(*n)
                    string.to_owned() + &append_str//error[E0369]: cannot add &String to &String ||Rust中不能直接拼接两个字符串引用，左侧需要是拥有所有权的String。
                    //修复方法：将左侧转换为拥有所有权的String   string.to_owned() + &append_str
                }
            };
            output.push(transformered);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
