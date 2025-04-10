// tests1.rs
//
// Tests are important to ensure that your code does what you think it should
// do. Tests can be run on this file with the following command: rustlings run
// tests1
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests1` or use the `hint` watch subcommand for a
// hint.



#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert() {
        assert!(true);
    }
}

#[cfg(test1)]
mod tests {
    #[test]
    fn you_can_assert() {
        assert!(false);
    }
}

#[cfg(test2)]
mod tests {
    #[test]
    fn you_can_assert() {
        assert!(1+1 == 2);
    }
}
/*
1. #[cfg(test)] 属性
    作用：#[cfg(test)] 是一个条件编译属性，用于指定模块或代码块仅在运行测试时编译。
    原理：当您运行 cargo test 命令时，Rust 编译器会启用 test 配置标志。
         #[cfg(test)] 属性告诉编译器，只有在启用 test 配置标志时，才编译带有该属性的代码。
    优势：隔离测试代码：测试代码不会包含在非测试编译中，避免增加二进制文件的大小。
        提高编译效率：在非测试编译中，测试代码不会被编译，从而加快编译速度。
2. mod tests 模块
    作用：mod tests 定义了一个名为 tests 的模块，用于组织测试代码。
    原理：在 Rust 中，测试代码通常放在 tests 模块中，以便更好地组织和管理。
          mod tests 模块可以包含多个测试函数，每个测试函数都使用 #[test] 属性进行标记。
    优势：代码组织：将测试代码集中在一个模块中，使代码结构更加清晰。
        隔离性：测试代码与普通代码分离，避免干扰。
3. #[test] 属性
    作用：#[test] 属性用于标记一个函数为测试函数。
    原理：当您运行 cargo test 命令时，Rust 测试框架会查找所有带有 #[test] 属性的函数，并执行它们。
        测试函数必须是无参数的，并且返回类型为 ()。
    优势：明确标识测试函数：使测试代码易于识别和管理。
        自动执行测试：测试框架会自动执行所有标记为 #[test] 的函数。
4. assert!() 宏
    作用：assert!() 宏用于验证布尔表达式是否为 true。
    原理：如果表达式为 true，测试通过。
        如果表达式为 false，测试失败，并输出相应的错误信息。
    优势：简洁的断言语法：提供了一种简单的方式来验证代码的正确性。
        清晰的错误信息：当测试失败时，会输出详细的错误信息，帮助开发者快速定位问题。
*/