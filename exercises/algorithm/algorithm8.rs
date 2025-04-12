/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/


#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        self.q1.enqueue(elem);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }
 
        // Move all elements except the last one from q1 to q2
        while self.q1.elements.len() > 1 {
            if let Ok(value) = self.q1.dequeue() {
                self.q2.enqueue(value);
            }
        }
 
        // The last element in q1 is the top of the stack
        let top = self.q1.dequeue().expect("This should not fail as we checked is_empty");
        //调用 dequeue 获取栈顶元素，并使用 expect，因为我们已经通过 is_empty 检查确保 q1 不为空。
        /*
        在 Rust 中，expect 是一个用于处理 Result 类型的方法，它的作用是在 Result 为 Err 时引发一个恐慌（panic），并输出指定的错误信息。
        使用 expect 的场景通常是在逻辑上不可能出现 Err 的情况下，作为一种简化处理的方式。
        根据逻辑，在调用 dequeue 之前，我们已经通过 is_empty 检查确保 q1 不为空。因此，理论上 dequeue 应该总是返回 Ok，而不会返回 Err。
        在这种情况下，使用 expect 是一种简洁的方式来处理 Result
         */
        // Swap q1 and q2 to maintain the invariant that q1 holds the stack elements
        std::mem::swap(&mut self.q1, &mut self.q2);
 
        Ok(top)
    }
    pub fn is_empty(&self) -> bool {
		//TODO
        self.q1.is_empty()
        //true
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}