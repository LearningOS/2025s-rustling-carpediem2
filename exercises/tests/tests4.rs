// tests4.rs
//
// Make sure that we're testing for the correct conditions!
//
// Execute `rustlings hint tests4` or use the `hint` watch subcommand for a
// hint.


struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    // Only change the test functions themselves
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle {width, height}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // This test should check if the rectangle is the size that we pass into its constructor
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // check width
        assert_eq!(rect.height, 20); // check height
    }

    /*
    虽然您使用了 #[should_panic] 属性，但没有指定预期的 panic 消息。
Rust 测试框架默认会检查 panic 的消息是否与预期一致，如果没有指定消息，它会认为 panic 不是预期的行为。
在 #[should_panic] 属性中指定 expected 参数，明确说明预期的 panic 消息。
     */
    #[test]
    #[should_panic(expected = "Rectangle width and height cannot be negative!")]
    fn negative_width() {
        // This test should check if program panics when we try to create rectangle with negative width
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic(expected = "Rectangle width and height cannot be negative!")]
    fn negative_height() {
        // This test should check if program panics when we try to create rectangle with negative height
        let _rect = Rectangle::new(10, -10);
    }
}
/*
运行了Rust测试，但遇到了两个失败的情况：negative_height和negative_width测试都失败了，因为它们预期程序会因为传入负的宽高而panic，但实际上程序确实panic了，但测试却标记为失败。这看起来有些矛盾，因为测试应该期望panic的发生。

让我仔细看看用户的测试代码。用户使用了#[should_panic]属性来标记这两个测试函数，这表示他们期望这些测试在执行时会触发panic。然而，测试结果却显示这两个测试失败了，尽管程序确实panic了。

可能的原因是，当测试函数因为panic而失败时，Rust的测试框架会认为测试没有通过，除非明确指定了#[should_panic]。但根据用户的描述，他们已经添加了#[should_panic]属性，所以理论上测试应该通过。

让我再检查一下用户的代码。在negative_height和negative_width测试函数中，用户确实添加了#[should_panic]属性。那为什么测试还会失败呢？

哦，可能用户在使用#[should_panic]时，没有正确指定panic的消息。在Rust中，#[should_panic]属性可以有一个可选的参数，用于指定预期的panic消息。如果panic的消息与预期不符，测试仍然会失败。

在用户提供的代码中，Rectangle::new函数在宽度或高度为负数时，会panic并显示消息"Rectangle width and height cannot be negative!"。然而，在测试函数中，用户没有指定预期的panic消息。因此，即使程序panic了，但panic的消息可能与测试框架预期的默认消息不匹配，导致测试失败。

为了解决这个问题，用户应该在#[should_panic]属性中指定预期的panic消息。这样，测试框架就会检查panic的消息是否与预期一致，从而正确标记测试为通过。

总结来说，用户需要在#[should_panic]属性中添加expected参数，指定预期的panic消息为"Rectangle width and height cannot be negative!"。这样，测试框架就会正确识别panic，并标记测试为通过。
*/