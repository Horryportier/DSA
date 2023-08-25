#[allow(dead_code)]
struct StackArr<T> {
    list: Vec<T>,
}

/// nothing to say most of the stack functions are easily simulated by Vec
#[allow(dead_code)]
impl<T> StackArr<T> {
    fn new() -> StackArr<T> {
        StackArr { list: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.list.push(item)
    }

    fn pop(&mut self) -> Option<T> {
        self.list.pop()
    }

    fn top(&mut self) -> Option<T> {
        self.list.pop()
    }

    fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    fn size(&self) -> usize {
        self.list.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::{Ok, Result};

    #[test]
    fn test_stack_simple() -> Result<()> {
        let mut stack = StackArr::<usize>::new();
        assert_eq!(stack.is_empty(), true);

        let v: Vec<usize> = vec![1, 2, 3];
        for i in v.clone() {
            stack.push(i)
        }
        assert_eq!(v, stack.list);
        assert_eq!(Some(3), stack.pop());
        assert_eq!(Some(2), stack.top());
        assert_eq!(1, stack.size());

        Ok(())
    } 
}
