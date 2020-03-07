use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Ref;

pub struct LinkedQueue<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    item: T,
    next: Link<T>,
}

pub struct ConsumerIter<T>(LinkedQueue<T>);

pub struct Iter<'a, T> {
    next: Option<Ref<'a, Node<T>>>,
}

impl<T> LinkedQueue<T> {
    pub fn new() -> Self {
        LinkedQueue { head: None, tail: None, }
    }

    pub fn is_empty(&self) -> bool {
        match self.head {
            None => true,
            Some(_) => false,
        }
    }

    pub fn enqueue(&mut self, item: T) {
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

    pub fn dequeue(&mut self) -> Option<T> {
        use std::mem;
        use mem::MaybeUninit;

        match self.head.take() {
            None => None,
            Some(first_node) => {
                unsafe {
                    let item =
                        mem::replace(&mut first_node.borrow_mut().item, MaybeUninit::uninit().assume_init());

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

    pub fn into_iter(self) -> ConsumerIter<T> {
        ConsumerIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_ref().map(|node| { node.borrow() })
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = Ref<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            let (next, item) = Ref::map_split(node, |node| {
                (&node.next, &node.item)
            });

            //TODO: So far it has been impossible to solve the life time issues
            //here. If possible find a way to implement RUST iterators
            //without using the `unsafe` feature.
            //let pipo = if next.is_some() {
            //    Some(Ref::map(next.as_ref().unwrap().borrow(), |node| node))
            //} else {
            //    None
            //};

            //self.next = pipo;

            item
        })
    }
}

impl<T> Iterator for ConsumerIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.dequeue()
    }
}

impl<T> Drop for LinkedQueue<T> {
    fn drop(&mut self) {
        let mut head_link = self.head.take();

        while let Some(boxed_node) = head_link {
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

    #[test]
    fn into_iter_should_consume_the_queue() {
        let mut queue = LinkedQueue::new();

        for i in 0..25 {
            queue.enqueue(i);
        }

        let mut k = 0;

        for v in queue.into_iter() {
            assert!(v == k);

            k += 1;
        }
    }
}
