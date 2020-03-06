pub struct LinkedStack<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    item: T,
    next: Link<T>,
}

pub struct ConsumerIter<T>(LinkedStack<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> LinkedStack<T> {
    pub fn new() -> Self {
        LinkedStack { head: None }
    }

    pub fn is_empty(&self) -> bool {
        match self.head {
            None => true,
            Some(_) => false,
        }
    }

    pub fn push(&mut self, item: T) {
        let new_node = Box::new(Node {
            item,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.item)
            }
        }
    }

    pub fn into_iter(self) -> ConsumerIter<T> {
        ConsumerIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_ref().map::<&Node<T>, _>(|node| &node) }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node)
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.item
        })
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);
            &node.item
        })
    }
}

impl<T> Iterator for ConsumerIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> Drop for LinkedStack<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();
        while let Some(mut boxed_node) = current_link {
            current_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use  crate::algorithms::linked_list_stack::LinkedStack;

    #[test]
    fn interface_operations_should_work_as_expected() {
        let mut stack = LinkedStack::<u32>::new();

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
        assert!(stack.pop() == None);
        assert!(stack.is_empty());
    }

    use  crate::algorithms::linked_list_stack::ConsumerIter;

    #[test]
    fn into_iter_should_consume_the_stack() {
        let mut stack = LinkedStack::<u32>::new();

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
        let mut stack = LinkedStack::<u32>::new();

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
        let mut stack = LinkedStack::<u32>::new();

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
