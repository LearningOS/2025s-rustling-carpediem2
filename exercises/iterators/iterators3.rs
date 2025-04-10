// iterators3.rs
//
// This is a bigger exercise than most of the others! You can do it! Here is
// your mission, should you choose to accept it:
// 1. Complete the divide function to get the first four tests to pass.
// 2. Get the remaining tests to pass by completing the result_with_list and
//    list_of_results functions.
//
// Execute `rustlings hint iterators3` or use the `hint` watch subcommand for a
// hint.


//搞了一个小时，后面继续复习！！！
#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    //todo!();
    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }
    if a % b != 0 {
        return Err(DivisionError::NotDivisible(NotDivisibleError {
            dividend: a,
            divisor: b,
        }));
    }
    Ok(a / b)
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output: Ok([1, 11, 1426, 3])
fn result_with_list() -> Result<Vec<i32>, DivisionError> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results: Result<Vec<_>, _> = numbers.into_iter().map(|n| divide(n, 27)).collect();
    /*numbers.into_iter()
    .map(|n| divide(n, 27))
    .collect()*/
    division_results
}
/*
遇到了测试失败的问题，具体是在test_result_with_list测试中，期望的结果是Ok([1, 11, 1426, 3])，但实际得到的是[Ok(1), Ok(11), Ok(1426), Ok(3)]。这表明函数返回的类型或结构仍然不符合预期。

希望函数result_with_list返回一个单一的Ok值，其内部包含一个列表，即Result<Vec<i32>, DivisionError>。然而，测试失败的原因可能是函数实际上返回了一个包含多个Ok值的列表，即Vec<Result<i32, DivisionError>>，而不是一个单一的Ok值。

为了修复这个问题，需要确保函数返回的类型是Result<Vec<i32>, DivisionError>，而不是Vec<Result<i32, DivisionError>>。这意味着在收集结果时，应该先将所有结果收集到一个Vec<i32>中，然后再将整个列表包裹在一个Ok中。
*/

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output: [Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() -> Vec<Result<i32, DivisionError>> {
    let numbers = vec![27, 297, 38502, 81];
    //let division_results = numbers.into_iter().map(|n| divide(n, 27));
    numbers.into_iter().map(|n| divide(n, 27)).collect::<Vec<Result<i32, DivisionError>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            format!("{:?}", list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}
