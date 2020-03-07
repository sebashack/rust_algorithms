use std::mem;

pub struct LinkedQueue<'a, T> {
    head: Link<T>,
    tail: Option<&'a mut Node<T>>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    item: T,
    next: Link<T>,
}

impl<'a, T> LinkedQueue<'a, T> {
    pub fn new() -> Self {
        LinkedQueue { head: None, tail: None, }
    }

    pub fn push(&'a mut self, item: T) {
        let new_tail = Box::new(Node {
            item,
            next: None,
        });

        let new_tail = match self.tail.take() {
            Some(old_tail) => {
                old_tail.next = Some(new_tail);
                old_tail.next.as_mut().map(|node| &mut **node)
            }
            None => {
                self.head = Some(new_tail);
                self.head.as_mut().map(|node| &mut **node)
            }
        };

        self.tail = new_tail;
    }
}

//pub struct LinkedQueue<T> {
//    head: Link<T>,
//    tail: Link<T>,
//}
//
//type Link<T> = Option<Rc<RefCell<Node<T>>>>;
//
//struct Node<T> {
//    item: T,
//    next: Link<T>,
//}
//
//
//impl<T> LinkedQueue<T> {
//    pub fn new() -> Self {
//        LinkedQueue { head: None, tail: None, }
//    }
//
//    pub fn is_empty(&self) -> bool {
//        match self.head {
//            None => true,
//            Some(_) => false,
//        }
//    }
//
//    pub fn enqueue(&mut self, item: T) {
//        let new_node = Rc::new(RefCell::new(Node {
//            item,
//            next: None,
//        }));
//
//        let previous_tail = self.tail.take();
//        self.tail = Some(Rc::clone(&new_node));
//
//        match &self.head {
//            None => {
//                self.head = Some(Rc::clone(&new_node));
//            }
//            Some(_) => {
//                if let Some(node) = previous_tail {
//                    node.borrow_mut().next = Some(Rc::clone(&new_node));
//                }
//            }
//        }
//    }
//
//    pub fn dequeue(&mut self) -> Option<T> {
//        use std::mem;
//        use mem::MaybeUninit;
//
//        match self.head.take() {
//            None => None,
//            Some(first_node) => {
//                unsafe {
//                    let item =
//                        mem::replace(&mut first_node.borrow_mut().item, MaybeUninit::uninit().assume_init());
//
//                    if let Some(next_node) = &first_node.borrow().next {
//                        self.head = Some(Rc::clone(next_node));
//                    } else {
//                        self.head = None;
//                        self.tail = None;
//                    }
//
//                    Some(item)
//                }
//            }
//        }
//    }
//}
//
//impl<T> Drop for LinkedQueue<T> {
//    fn drop(&mut self) {
//        let mut head_link = self.head.take();
//
//        while let Some(boxed_node) = head_link {
//            head_link = boxed_node.borrow_mut().next.take();
//        }
//
//        self.tail.take();
//    }
//}

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

        for i in 0..50 {
            queue.enqueue(i);
            queue.dequeue();
        }

        assert!(queue.is_empty());

        for i in 0..1000 {
            queue.enqueue(i);
        }

        for i in 0..1000 {
            queue.dequeue();
        }

        assert!(queue.is_empty());

        let mut queue = LinkedQueue::new();

        for i in 0..100 {
            queue.enqueue(i);
        }

        for i in 0..100 {
            assert!(queue.dequeue() == Some(i));
        }

        for i in 0..50 {
            queue.enqueue(i);
            queue.dequeue();
        }

        for i in 0..1000 {
            queue.enqueue(i);
        }

        for i in 0..1000 {
            assert!(queue.dequeue() == Some(i));
        }

        assert!(queue.is_empty());
    }
}
