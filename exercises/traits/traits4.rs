// traits4.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits4` or use the `hint` watch subcommand for a
// hint.



pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn compare_license_types<T: Licensed,U: Licensed>(software: T, software_two: U) -> bool {
    software.licensing_info() == software_two.licensing_info()
}
/*
泛型与特质约束：compare_license_types函数使用两个泛型类型参数T和U，它们都被Licensed特质约束。这意味着T和U可以是任何实现了Licensed特质的类型。
灵活性：通过使用泛型，函数可以接受SomeSoftware和OtherSoftware的实例（或任何其他实现了Licensed特质的类型），并比较它们的许可信息。这使得函数具有可重用性和类型安全性。
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(some_software, other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(other_software, some_software));
    }
}
