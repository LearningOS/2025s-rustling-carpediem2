// traits3.rs
//
// Your task is to implement the Licensed trait for both structures and have
// them return the same information without writing the same function twice.
//
// Consider what you can add to the Licensed trait.
//
// Execute `rustlings hint traits3` or use the `hint` watch subcommand for a
// hint.



/*
为了避免重复编写代码，可以考虑在trait中提供一个默认实现。这样，所有实现该trait的结构体都可以使用默认的实现，除非需要特殊的处理。

因此，正确的实现方式应该是：

1.在Licensed trait中为licensing_info方法提供一个默认实现，返回固定的许可信息字符串。
2.为SomeSoftware和OtherSoftware结构体实现Licensed trait，使用默认的实现。
这样，两个结构体都会返回相同的许可信息，而无需重复编写代码。
*/
pub trait Licensed {
    fn licensing_info(&self) -> String{//licensing_info方法接受一个不可变的self引用，并返回一个String
        String::from("Some information")
    }
    
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // Don't edit this line
impl Licensed for OtherSoftware {} // Don't edit this line

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }

}
