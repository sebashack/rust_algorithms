pub struct LinkedStack {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    item: i32,
    next: Link,
}

impl LinkedStack {
    pub fn new() -> Self {
        LinkedStack { head: None }
    }

    pub fn is_empty(&self) -> bool {
        match self.head {
            None => true,
            Some(_) => false,
        }
    }

    pub fn push(&mut self, item: i32) {
        let new_node = Box::new(Node {
            item,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.item)
            }
        }
    }
}

impl Drop for LinkedStack {
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
        let mut stack = LinkedStack::new();

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
