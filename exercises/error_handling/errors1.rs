// errors1.rs
//
// This function refuses to generate text to be printed on a nametag if you pass
// it an empty string. It'd be nicer if it explained what the problem was,
// instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Result` that can be used to express error conditions. Let's use
// it!
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.



//pub fn generate_nametag_text(name: String) -> Option<String> {
//   if name.is_empty() {
  //      // Empty names aren't allowed.
    //    None
    //} else {
      //  Some(format!("Hi! My name is {}", name))
    //}
//}
pub fn generate_nametag_text(name: String) -> Result<String,String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        // 返回错误并附带解释信息
        Err("`name` was empty; it must be nonempty.".into())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}
//将返回类型从 Option<String> 改为 Result<String, String>
//使用 Ok() 包装成功结果，Err() 包装错误信息,保持原有逻辑结构，但改变返回值构造方式,测试用例直接匹配新的 Result 类型

//Result 类型比 Option 更适合需要区分不同错误类型的情况,使用 Err("message".into()) 可以携带详细的错误解释信息
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
