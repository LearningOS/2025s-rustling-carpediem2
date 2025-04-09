// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.


//完成simple_option测试：使用if let语句来解构optional_target，获取其内部的String值。在if let块内部进行断言，确保解构出的值与target相等。
//完成layered_option测试：使用while let语句来处理optional_integers.pop()返回的Option<Option<i8>>。在while let块内部进行断言，确保解构出的值与cursor相等，并更新cursor的值。
#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
        //if let Some(word) = optional_target 会尝试解构 optional_target
    //如果解构成功（即 optional_target 是 Some 类型），则将内部值绑定到 word 变量
    //在代码块内部进行断言，验证解构出的值是否等于 target
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }
        //optional_integers.pop() 返回 Option<Option<i8>>（因为 Vec::pop() 本身返回 Option<&T>，而这里的 T 是 Option<i8>）
        //while let Some(Some(integer)) = ... 进行双重解构：外层 Some 来自 Vec::pop() 的结果,内层 Some 来自 Option<i8> 的解构
        //如果解构成功，则将内部值绑定到 integer 变量,在循环体内进行断言，并更新 cursor 的值
        assert_eq!(cursor, 0);
    }
}
