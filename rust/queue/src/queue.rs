
#[derive(Debug, Default)]
struct Queue {
    vec: Vec<usize>
}

impl Queue{
    fn new(vec: Vec<usize>) -> Queue {
        Queue { vec }
    }

    fn enqueue(&mut self, item: usize) {
        self.vec.push(item)
    }

    fn dequeue(&mut self) -> Option<usize> {
        let r = self.vec.get(0).copied();
        self.vec.remove(0);
        r 
    }

    fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    fn peek(&mut self) -> Option<usize> {
        self.vec.get(0).copied()
    }

    

    fn size(&self) -> usize {
        self.vec.len()
    }

    fn clear(&mut self) {
        self.vec.clear()
    }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t() {
        let mut q = Queue::new(vec![1,2,3]);
        
        assert_eq!(3, q.size());
        q.enqueue(4);
        assert_eq!(Some(1), q.dequeue());
        assert_eq!(vec![2,3,4], q.vec);
        assert_eq!(Some(2), q.peek())
    }
}
