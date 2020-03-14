pub struct PriorityQueue<T> {
    queue: Vec<Option<T>>,
    last: usize,
    is_max: bool,
}

use std::fmt;

impl<T> PriorityQueue<T>
where
    T: Ord,
{
    pub fn new(is_max: bool) -> Self {
        let mut queue = Vec::with_capacity(2);

        queue.insert(0, None);

        PriorityQueue {
            queue,
            last: 0,
            is_max,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.last == 0
    }

    pub fn insert(&mut self, key: T) {
        let capacity = self.queue.capacity();
        if self.last == capacity {
            self.resize(2 * capacity);
        }

        self.last += 1;
        insert(&mut self.queue, self.last, Some(key));

        self.swim(self.last);
    }

    pub fn delete(&mut self) -> Option<T> {
        let max = self.queue[1].take();
        self.queue.swap(1, self.last);
        self.last -= 1;
        self.sink(1);

        let capacity = self.queue.capacity();
        if self.last == capacity / 4 {
            self.resize(capacity / 2);
        }

        max
    }

    fn swim(&mut self, k: usize) {
        let mut k = k;

        loop {
            let parent = self.queue[k / 2].as_ref();
            let child = self.queue[k].as_ref();

            if k > 1 && self.compare(parent.unwrap(), child.unwrap()) {
                self.queue.swap(k, k / 2);
                k = k / 2;
            } else {
                break;
            }
        }
    }

    fn compare(&self, a: &T, b: &T) -> bool {
        if self.is_max {
            a < b
        } else {
            a > b
        }
    }

    fn sink(&mut self, k: usize) {
        let mut k = k;

        while 2 * k <= self.last {
            let mut j = 2 * k;
            let parent = self.queue[k].as_ref();
            let l_child = self.queue[j].as_ref();
            let r_child = self.queue[j + 1].as_ref();

            if j < self.last && self.compare(l_child.unwrap(), r_child.unwrap()) {
                j += 1;
            }

            if self.compare(parent.unwrap(), self.queue[j].as_ref().unwrap()) {
                self.queue.swap(k, j);
                k = j;
            } else {
                break;
            }
        }
    }

    fn resize(&mut self, capacity: usize) {
        let mut new_queue = Vec::with_capacity(capacity);

        for i in 0..=self.last {
            insert(&mut new_queue, i, self.queue[i].take());
        }

        self.queue = new_queue;
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
    use crate::algorithms::priority_queue::PriorityQueue;

    #[test]
    fn interface_operations_for_max_priority_queue_should_work_as_expected() {
        let mut queue = PriorityQueue::<isize>::new(true);

        assert!(queue.is_empty());

        for v in 0..100 {
            queue.insert(v);
        }

        for v in 0..100 {
            assert!(queue.delete() == Some(99 - v));
        }

        assert!(queue.is_empty());

        queue.insert(1);
        queue.insert(10);
        queue.insert(20);
        queue.insert(5);
        queue.insert(-10);
        queue.insert(-30);
        queue.insert(-2);
        queue.insert(1);
        queue.insert(5);

        assert!(queue.delete().unwrap() == 20);
        assert!(queue.delete().unwrap() == 10);
        assert!(queue.delete().unwrap() == 5);
        assert!(queue.delete().unwrap() == 5);
        assert!(queue.delete().unwrap() == 1);
        assert!(queue.delete().unwrap() == 1);
        assert!(queue.delete().unwrap() == -2);
        assert!(queue.delete().unwrap() == -10);
        assert!(queue.delete().unwrap() == -30);
        assert!(queue.is_empty());
    }

    #[test]
    fn interface_operations_for_min_priority_queue_should_work_as_expected() {
        let mut queue = PriorityQueue::<isize>::new(false);

        assert!(queue.is_empty());

        for v in 0..100 {
            queue.insert(v);
        }

        for v in 0..100 {
            assert!(queue.delete() == Some(v));
        }

        assert!(queue.is_empty());

        queue.insert(1);
        queue.insert(10);
        queue.insert(20);
        queue.insert(5);
        queue.insert(-10);
        queue.insert(-30);
        queue.insert(-2);
        queue.insert(1);
        queue.insert(5);

        assert!(queue.delete().unwrap() == -30);
        assert!(queue.delete().unwrap() == -10);
        assert!(queue.delete().unwrap() == -2);
        assert!(queue.delete().unwrap() == 1);
        assert!(queue.delete().unwrap() == 1);
        assert!(queue.delete().unwrap() == 5);
        assert!(queue.delete().unwrap() == 5);
        assert!(queue.delete().unwrap() == 10);
        assert!(queue.delete().unwrap() == 20);

        assert!(queue.is_empty());
    }
}
