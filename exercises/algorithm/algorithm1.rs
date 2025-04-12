/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    //后面继续复习，和cpp差别明显
	pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> Self 
    where 
        T: Ord + Clone,//添加 Clone 约束：在 LinkedList<T> 的实现中，为类型参数 T 添加 Clone 约束。
	{
		//TODO
		/*Self {
            length: 0,
            start: None,
            end: None,
        }*/
        let mut merged_list = LinkedList::new();
 
        let mut a_ptr = list_a.start;
        let mut b_ptr = list_b.start;
 
        let mut merged_tail: Option<NonNull<Node<T>>> = None;
 
        // 合并两个链表
        while let (Some(a_node), Some(b_node)) = (a_ptr, b_ptr) {
            unsafe {
                let a_val = (*a_node.as_ptr()).val.clone();
                let b_val = (*b_node.as_ptr()).val.clone();
 
                let new_node = if a_val <= b_val {
                    a_ptr = (*a_node.as_ptr()).next;
                    Box::new(Node::new(a_val))
                } else {
                    b_ptr = (*b_node.as_ptr()).next;
                    Box::new(Node::new(b_val))
                };
 
                let new_node_ptr = NonNull::new(Box::into_raw(new_node));
                if merged_tail.is_none() {
                    merged_list.start = new_node_ptr;
                    merged_tail = new_node_ptr;
                } else {
                    (*merged_tail.unwrap().as_ptr()).next = new_node_ptr;
                    merged_tail = new_node_ptr;
                }
            }
        }
 
        // 处理剩余节点
        let mut remaining_ptr = if a_ptr.is_some() { a_ptr } else { b_ptr };
        while let Some(node) = remaining_ptr {
            unsafe {
                let new_node = Box::new(Node::new((*node.as_ptr()).val.clone()));
                let new_node_ptr = NonNull::new(Box::into_raw(new_node));
                if merged_tail.is_none() {
                    merged_list.start = new_node_ptr;
                    merged_tail = new_node_ptr;
                } else {
                    (*merged_tail.unwrap().as_ptr()).next = new_node_ptr;
                    merged_tail = new_node_ptr;
                }
            }
            remaining_ptr = unsafe { (*remaining_ptr.unwrap().as_ptr()).next };
        }
 
        // 更新链表长度
        merged_list.length = list_a.length + list_b.length;
 
        // 找到链表的末尾以设置 end 指针（可选，因为 end 指针在合并过程中未使用）
        let mut current_ptr = merged_list.start;
        let mut end_ptr = None;
        while let Some(node_ptr) = current_ptr {
            unsafe {
                end_ptr = Some(node_ptr);
                current_ptr = (*node_ptr.as_ptr()).next;
            }
        }
        merged_list.end = end_ptr;
 
        merged_list
    
	}
}
/*
初始化：创建一个新的空链表 merged_list 用于存储合并结果。
合并过程：使用两个指针 a_ptr 和 b_ptr 分别遍历 list_a 和 list_b，比较节点值后将较小的节点添加到 merged_list 中。
处理剩余节点：当一个链表遍历完后，将另一个链表的剩余部分直接添加到 merged_list 中。
更新长度：合并完成后，更新 merged_list 的长度。
设置尾指针（可选）：遍历 merged_list 以设置尾指针（在合并过程中未使用，但可用于其他操作）。

使用 NonNull<Node<T>> 和 Box::into_raw 来管理节点内存，确保内存安全。
在合并过程中，直接操作原始指针，避免不必要的内存分配和释放。

*/

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}