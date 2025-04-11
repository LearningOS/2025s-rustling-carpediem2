// cow1.rs
//
// This exercise explores the Cow, or Clone-On-Write type. Cow is a
// clone-on-write smart pointer. It can enclose and provide immutable access to
// borrowed data, and clone the data lazily when mutation or ownership is
// required. The type is designed to work with general borrowed data via the
// Borrow trait.
//
// This exercise is meant to show you what to expect when passing data to Cow.
// Fix the unit tests by checking for Cow::Owned(_) and Cow::Borrowed(_) at the
// TODO markers.

/*
本练习探讨 Cow 或 Clone-On-Write 类型。Cow 是一个写时克隆的智能指针。它可以包含并提供对借用数据的不可变访问，并在需要更改或所有权时延迟克隆数据。该类型旨在通过 Borrow 特征处理一般借用数据。
//
本练习旨在向您展示将数据传递给 Cow 时会发生什么。通过在 TODO 标记处检查 Cow：：Owned（_） 和 Cow：：Borrowed（_） 来修复单元测试。
*/
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.


//Cow有两种状态：Borrowed和Owned。当需要修改数据时，Cow会从Borrowed转为Owned，即克隆数据。

use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;
/*
reference_mutation测试：原始数据是slice的引用（Cow::Borrowed），
    调用abs_all后，因为需要修改数据，所以会触发克隆，变成Owned。
    所以这里匹配Cow::Owned是正确的。

reference_no_mutation测试：原始数据是slice的引用，调用abs_all后，数据不需要修改，
    所以Cow保持Borrowed状态。所以这里应该匹配Cow::Borrowed。

owned_no_mutation测试：原始数据已经是Owned（因为Cow::from(slice)中的slice是vec，
    所以Cow直接拥有它），调用abs_all后，数据不需要修改，所以仍然是Owned。
    所以这里应该匹配Cow::Owned。

owned_mutation测试：原始数据是Owned，调用abs_all时，虽然进行了修改，但因为已经是Owned状态，
    所以不需要克隆。所以结果还是Owned。这里应该匹配Cow::Owned。
*/
    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            // TODO
            Cow::Borrowed(_) => Ok(()),// 没有修改时保持借用状态
            _ =>Err("Expected borrowed value")
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // We can also pass `slice` without `&` so Cow owns it directly. In this
        // case no mutation occurs and thus also no clone, but the result is
        // still owned because it was never borrowed or mutated.
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            // TODO
            Cow::Owned(_) => Ok(()),// 初始即为拥有状态且未修改
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // Of course this is also the case if a mutation does occur. In this
        // case the call to `to_mut()` returns a reference to the same data as
        // before.
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            // TODO
            Cow::Owned(_) => Ok(()),// 拥有状态修改后仍保持拥有
            _ => Err("Expected owned value"),
        }
    }
}

/*
reference_no_mutation 测试中，由于输入全为非负数，Cow保持Borrowed状态
owned_no_mutation 测试中，初始即为Owned状态且未触发克隆，保持Owned状态
owned_mutation 测试中，虽然进行了修改但原始数据已经是Owned状态，无需克隆，最终仍返回Owned状态
*/
