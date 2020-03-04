pub struct ArrayStack<T> {
    container: Vec<Option<T>>,
    current: usize,
}

impl<T> ArrayStack<T> {
    pub fn new() -> Self {
        let container = Vec::with_capacity(1);

        ArrayStack {
            container,
            current: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.current == 0
    }

    pub fn push(&mut self, item: T) {
        let capacity = self.container.capacity();
        if self.current == capacity {
            self.resize(2 * capacity);
        }

        insert(&mut self.container, self.current, Some(item));
        self.current += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.current == 0 {
            None
        } else {
            self.current -= 1;

            let item = self.container[self.current].take();
            let capacity = self.container.capacity();

            if self.current == capacity / 4 {
                self.resize(capacity / 2);
            }

            item
        }
    }

    fn resize(&mut self, capacity: usize) {
        let mut new_container = Vec::with_capacity(capacity);

        for i in 0..self.current {
            insert(&mut new_container, i, self.container[i].take());
        }

        self.container = new_container;
    }
}

fn insert<T>(v: &mut Vec<Option<T>>, i: usize, item: Option<T>) {
    if v.len() <= i {
        v.insert(i, item);
    } else {
        v[i] = item;
    }
}

#[cfg(test)]
mod tests {
    use  crate::algorithms::array_stack::ArrayStack;

    #[test]
    fn interface_operations_should_work_as_expected() {
        let mut stack = ArrayStack::<u32>::new();

        assert!(stack.is_empty());
        assert!(stack.pop() == None);

        stack.push(0);

        assert!(stack.pop() == Some(0));

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert!(stack.pop() == Some(3));
        assert!(stack.pop() == Some(2));
        assert!(stack.pop() == Some(1));

        assert!(stack.is_empty());

        for i in 0..99 {
            stack.push(i);
            stack.pop();
        }

        assert!(stack.is_empty());

        for i in 0..99 {
            stack.push(i);
        }

        for i in 0..99 {
           assert!(stack.pop() == Some(98 - i));
        }

        assert!(stack.is_empty());
    }
}
