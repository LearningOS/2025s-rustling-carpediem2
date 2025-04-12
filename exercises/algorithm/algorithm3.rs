/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

/*
泛型约束：
    sort 函数使用了泛型 T，并约束 T 必须实现 Ord trait，这样我们就可以比较元素的大小。
插入排序算法：
    外层循环从第二个元素开始遍历数组。
    内层循环将当前元素与前面的元素进行比较，如果前面的元素大于当前元素，则交换它们的位置。
    继续向前比较，直到找到一个不大于当前元素的元素或者到达数组的开头。
*/
fn sort<T>(array: &mut [T]) where T: Ord,{
	//TODO
    let len = array.len();
    for i in 1..len {
        let mut j = i;
        while j > 0 && array[j - 1] > array[j] {
            array.swap(j, j - 1);
            j -= 1;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}