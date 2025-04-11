// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// Your task is to complete this implementation in order for the line `let p =
// Person::from("Mark,20")` to compile Please note that you'll need to parse the
// age component into a `usize` with something like `"4".parse::<usize>()`. The
// outcome of this needs to be handled appropriately.
//
// Steps:
// 1. If the length of the provided string is 0, then return the default of
//    Person.
// 2. Split the given string on the commas present in it.
// 3. Extract the first element from the split operation and use it as the name.
// 4. If the name is empty, then return the default of Person.
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age.
// If while parsing the age, something goes wrong, then return the default of
// Person Otherwise, then return an instantiated Person object with the results
/*
你的任务是完成这个实现，以便编译 'let p =Person：：from（“Mark，20”）' 行。请注意，你需要将 age 组件解析为类似于 '“4”.parse：：（）' 的 'usize'。其结果需要得到适当的处理。
//
步骤：
// 1.如果提供的字符串的长度为 0，则返回默认值
人。
// 2.将给定的字符串拆分为其中的逗号。
// 3.从 split作中提取第一个元素并将其用作名称。
// 4.如果名称为空，则返回默认值 Person。
// 5.从 split作中提取另一个元素并将其解析为
'usize' 作为年龄。
如果在解析年龄时出现问题，则返回默认值 Person Otherwise，然后返回实例化的 Person 对象及其结果
*/


impl From<&str> for Person {
    fn from(s: &str) -> Person {
        // 处理空字符串情况
        if s.is_empty() {
            return Person::default();//直接返回默认值
        }
 
        // 分割字符串
        //使用split_once确保只分割第一个逗号，避免多个逗号导致的错误
        let split_result = s.split_once(',');
        let (name_part, age_part) = match split_result {
            Some((name, age)) => (name, age),
            None => return Person::default(), // 没有逗号的情况
        };
 
        // 处理姓名部分
        //使用trim()去除前后空格，检查是否为空
        let name = match name_part.trim() {
            "" => return Person::default(), // 名称为空
            name => name.to_string(),
        };
 
        // 处理年龄部分
        //使用parse::<usize>()进行类型安全的转换
        let age = match age_part.trim().parse::<usize>() {
            Ok(age) => age,
            Err(_) => return Person::default(), // 解析失败
        };
 
        // 所有验证通过，创建Person
        Person { name, age }
    }
}

fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an
        // error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
