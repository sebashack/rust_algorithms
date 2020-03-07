use std::ptr;

pub struct LinkedQueue<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    item: T,
    next: Link<T>,
}

pub struct IntoIter<T>(LinkedQueue<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> LinkedQueue<T> {
    pub fn new() -> Self {
        LinkedQueue { head: None, tail: ptr::null_mut(), }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn enqueue(&mut self, item: T) {
        let mut new_tail = Box::new(Node {
            item,
            next: None,
        });

        let raw_tail: *mut _ = &mut *new_tail;

        if self.tail.is_null() {
            self.head = Some(new_tail);
        } else {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        }

        self.tail = raw_tail;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            head.item
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

impl<T> Drop for LinkedQueue<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.dequeue()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.item
        })
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

        //At this point it is impossible to use the queue since it has been moved.
    }

    #[test]
    fn iter_should_traverse_the_queue_without_consuming_it() {
        let mut queue = LinkedQueue::new();

        for i in 0..25 {
            queue.enqueue(i);
        }

        let mut k = 0;

        for v in queue.iter() {
            assert!(*v == k);

            k += 1;
        }

        for i in 0..25 {
            queue.dequeue();
        }

        assert!(queue.is_empty());

        for i in 0..25 {
            queue.enqueue(i);
        }

        k = 0;

        for v in queue.iter() {
            assert!(*v == k);

            k += 1;
        }
    }

    #[test]
    fn iter_should_traverse_the_queue_while_mutating_it() {
        let mut queue = LinkedQueue::new();

        for i in 0..25 {
            queue.enqueue(i);
        }

        for v in queue.iter_mut() {
            *v = 0;
        }

        for v in queue.iter() {
            assert!(*v == 0);
        }
    }
}
