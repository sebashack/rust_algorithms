pub struct ArrayStack {
    container: Vec<Option<i32>>,
    current: usize,
}

impl ArrayStack {
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

    pub fn push(&mut self, item: i32) {
        let capacity = self.container.capacity();
        if self.current == capacity {
            self.resize(2 * capacity);
        }

        insert(&mut self.container, self.current, Some(item));
        self.current += 1;
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.current == 0 {
            None
        } else {
            self.current -= 1;

            let item = self.container[self.current];
            self.container[self.current] = None;

            let capacity = self.container.capacity();
            if self.current == capacity / 4 {
                self.resize(capacity / 2);
            }

            item
        }
    }

    fn resize(&mut self, capacity: usize) {
        let mut new_container = Vec::with_capacity(capacity);

        for (i, v) in self.container.iter().enumerate() {
            insert(&mut new_container, i, *v);
        }

        self.container = new_container;
    }
}

fn insert(v: &mut Vec<Option<i32>>, i: usize, item: Option<i32>) {
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
        let mut stack = ArrayStack::new();

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
    }
}
