use std::slice::IterMut as IterMutVec;

pub struct ArrayStack<T> {
    container: Vec<Option<T>>,
    current: usize,
}

pub struct ConsumerIter<T>(ArrayStack<T>);

pub struct Iter<'a, T> {
    container: &'a Vec<Option<T>>,
    next: usize,
}

pub struct IterMut<'a, T>(IterMutVec<'a, Option<T>>);

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

    pub fn into_iter(self) -> ConsumerIter<T> {
        ConsumerIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            container: &self.container,
            next: self.current,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut(self.container.iter_mut())
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

impl<T> Iterator for ConsumerIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == 0 {
            None
        } else {
            self.next -= 1;
            self.container[self.next].as_ref()
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            None => None,
            Some(item) => item.as_mut(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::array_stack::ArrayStack;

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

    #[test]
    fn into_iter_should_consume_the_stack() {
        let mut stack = ArrayStack::<u32>::new();

        for i in 0..25 {
            stack.push(i);
        }

        let mut k = 25;

        for v in stack.into_iter() {
            k -= 1;

            assert!(v == k);
        }

        //At this point it is impossible to use the stack since it has been moved.
    }

    #[test]
    fn iter_should_traverse_the_stack_without_consuming_it() {
        let mut stack = ArrayStack::<u32>::new();

        for i in 0..25 {
            stack.push(i);
        }

        let mut k = 25;

        for v in stack.iter() {
            k -= 1;

            assert!(*v == k);
        }

        k = 25;

        //After this point it is totally possible to keep using our stack since
        //our iterator is just borrowing immutable references.

        assert!(!stack.is_empty());

        for v in stack.iter() {
            k -= 1;

            assert!(*v == k);
        }
    }

    #[test]
    fn iter_should_traverse_the_stack_while_mutating_it() {
        let mut stack = ArrayStack::<u32>::new();

        for i in 1..26 {
            stack.push(i);
        }

        for v in stack.iter_mut() {
            *v = 0;
        }

        for v in stack.iter() {
            assert!(*v == 0);
        }
    }
}
