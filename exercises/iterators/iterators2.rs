// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.



// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars(); // 创建字符迭代器
    match c.next() {
        None => String::new(),// 空字符串直接返回
        Some(first) => {
            // 将首字母大写，并拼接剩余字符
            let mut capitalized = String::new();
            capitalized.push(first.to_uppercase().next().unwrap()); // 处理Unicode大写转换
            capitalized.push_str(c.as_str()); // 追加剩余字符
            capitalized
        }
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    //vec![]
    words.iter()
         .map(|word| capitalize_first(word)) // 对每个单词应用首字母大写
         .collect() // 收集到Vec<String>
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut result = String::new();
    for word in words{
        result.push_str(&capitalize_first(word)); // 直接拼接处理后的字符串
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}

/*
首字母大写函数 (capitalize_first)使用chars()迭代器逐个处理字符
    to_uppercase().next().unwrap() 处理Unicode大写转换（如'ß'→"SS"）
    chars.as_str() 获取剩余字符的字符串切片
字符串向量处理 (capitalize_words_vector)使用iter().map().collect()模式进行链式处理
    直接将处理结果收集到Vec<String>
字符串拼接处理 (capitalize_words_string)使用String::push_str()进行高效拼接
    保留原始空格元素（如输入中的" "会被保留为" "）
*/