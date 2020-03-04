use std::rc::Rc;
use std::cell::RefCell;

pub struct LinkedQueue {
    head: Link,
    tail: Link,
}

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    item: i32,
    next: Link,
}


impl LinkedQueue {
    pub fn new() -> Self {
        LinkedQueue { head: None, tail: None, }
    }

    pub fn is_empty(&self) -> bool {
        match self.head {
            None => true,
            Some(_) => false,
        }
    }

    pub fn enqueue(&mut self, item: i32) {
        let new_node = Rc::new(RefCell::new(Node {
            item,
            next: None,
        }));

        let previous_tail = self.tail.take();
        self.tail = Some(Rc::clone(&new_node));

        match &self.head {
            None => {
                self.head = Some(Rc::clone(&new_node));
            }
            Some(_) => {
                if let Some(node) = previous_tail {
                    node.borrow_mut().next = Some(Rc::clone(&new_node));
                }
            }
        }
    }

    pub fn dequeue(&mut self) -> Option<i32> {
        match self.head.take() {
            None => None,
            Some(first_node) => {
                let item = first_node.borrow().item;

                if let Some(next_node) = &first_node.borrow().next {
                    self.head = Some(Rc::clone(next_node));
                } else {
                    self.head = None;
                    self.tail = None;
                }

                Some(item)
            }
        }
    }
}

impl Drop for LinkedQueue {
    fn drop(&mut self) {
        let mut head_link = self.head.take();

        while let Some(mut boxed_node) = head_link {
            head_link = boxed_node.borrow_mut().next.take();
        }

        self.tail.take();
    }
}

#[cfg(test)]
mod tests {
    use  crate::algorithms::linked_list_queue::LinkedQueue;

    #[test]
    fn interface_operations_should_work_as_expected() {
        let mut queue = LinkedQueue::new();

        assert!(queue.is_empty());
        assert!(queue.dequeue() == None);

        queue.enqueue(0);

        assert!(queue.dequeue() == Some(0));

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(4);

        assert!(queue.dequeue() == Some(1));
        assert!(queue.dequeue() == Some(2));
        assert!(queue.dequeue() == Some(3));
        assert!(queue.dequeue() == Some(4));

        assert!(queue.is_empty());

        queue.enqueue(1);
        queue.dequeue();
        queue.enqueue(2);
        queue.dequeue();

        assert!(queue.is_empty());
    }
}
