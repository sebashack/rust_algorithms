pub struct LinkedStack<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    item: T,
    next: Link<T>,
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
}
