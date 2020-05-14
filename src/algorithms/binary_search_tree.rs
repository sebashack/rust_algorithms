use std::cell::RefCell;
use std::cmp::Ordering::{Equal, Greater, Less};

use crate::algorithms::linked_list_queue::LinkedQueue;

enum BST<K, V> {
    Node {
        key: K,
        val: V,
        left: Box<BST<K, V>>,
        right: Box<BST<K, V>>,
        count: usize,
    },
    Leaf,
}

impl<K, V> BST<K, V>
where
    K: Ord,
{
    pub fn new(key: K, val: V) -> Self {
        BST::Node {
            key,
            val,
            left: Box::new(BST::Leaf),
            right: Box::new(BST::Leaf),
            count: 1,
        }
    }

    pub fn is_leaf(&self) -> bool {
        use crate::algorithms::binary_search_tree::BST::{Leaf, Node};

        if let Leaf = self {
            true
        } else {
            false
        }
    }

    pub fn size(&self) -> usize {
        use crate::algorithms::binary_search_tree::BST::{Leaf, Node};

        match self {
            Leaf => 0,
            Node { count, .. } => *count,
        }
    }

    pub fn put(&mut self, k: K, v: V) {
        use crate::algorithms::binary_search_tree::BST::{Leaf, Node};

        match self {
            Leaf => *self = BST::new(k, v),
            Node {
                key,
                val,
                left,
                right,
                ..
            } => match k.cmp(key) {
                Less => left.put(k, v),
                Greater => right.put(k, v),
                Equal => return,
            },
        }

        match self {
            Leaf => {}
            Node {
                left, right, count, ..
            } => {
                *count = 1 + left.size() + right.size();
            }
        }
    }

    pub fn get(&self, k: K) -> Option<&V> {
        use crate::algorithms::binary_search_tree::BST::{Leaf, Node};

        match self {
            Leaf => None,
            Node {
                key,
                val,
                left,
                right,
                ..
            } => match &k.cmp(key) {
                Less => left.get(k),
                Greater => right.get(k),
                Equal => Some(&val),
            },
        }
    }

    pub fn min(&self) -> Option<&V> {
        use crate::algorithms::binary_search_tree::BST::{Leaf, Node};

        match self {
            Leaf => None,
            Node { key, val, left, .. } => {
                if left.is_leaf() {
                    Some(&val)
                } else {
                    left.min()
                }
            }
        }
    }

    pub fn max(&self) -> Option<&V> {
        use crate::algorithms::binary_search_tree::BST::{Leaf, Node};

        match self {
            Leaf => None,
            Node {
                key, val, right, ..
            } => {
                if right.is_leaf() {
                    Some(&val)
                } else {
                    right.max()
                }
            }
        }
    }

    pub fn keys(&self) -> LinkedQueue<&K> {
        use crate::algorithms::binary_search_tree::BST::{Leaf, Node};

        let mut q = LinkedQueue::<&K>::new();
        let mut rcq = RefCell::new(&mut q);

        inorder_keys(self, &mut rcq);

        q
    }

    pub fn values(&self) -> LinkedQueue<&V> {
        use crate::algorithms::binary_search_tree::BST::{Leaf, Node};

        let mut q = LinkedQueue::<&V>::new();
        let mut rcq = RefCell::new(&mut q);

        inorder_values(self, &mut rcq);

        q
    }
}

fn inorder_keys<'a, K, V>(node: &'a BST<K, V>, q: &RefCell<&mut LinkedQueue<&'a K>>) {
    use crate::algorithms::binary_search_tree::BST::{Leaf, Node};

    match node {
        Leaf => {}
        Node {
            key, left, right, ..
        } => {
            inorder_keys(left, q);
            q.borrow_mut().enqueue(key);
            inorder_keys(right, q);
        }
    }
}

fn inorder_values<'a, K, V>(node: &'a BST<K, V>, q: &RefCell<&mut LinkedQueue<&'a V>>) {
    use crate::algorithms::binary_search_tree::BST::{Leaf, Node};

    match node {
        Leaf => {}
        Node {
            val, left, right, ..
        } => {
            inorder_values(left, q);
            q.borrow_mut().enqueue(val);
            inorder_values(right, q);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::binary_search_tree::BST;
    use crate::algorithms::binary_search_tree::BST::{Leaf, Node};

    #[test]
    fn interface_operations_should_work_as_expected() {
        let mut bst = BST::Leaf;

        assert!(bst.size() == 0);

        bst.put(0, "v");

        assert!(bst.get(0) == Some(&"v"));
        assert!(bst.size() == 1);

        let mut bst = BST::<i32, &str>::new(0, "v");

        bst.put(-1, "vv");
        bst.put(3, "vvv");
        bst.put(-2, "vvvv");
        bst.put(5, "vvvvv");
        bst.put(7, "vvvvvvv");

        assert!(bst.size() == 6);
        assert!(bst.get(0) == Some(&"v"));
        assert!(bst.get(-1) == Some(&"vv"));
        assert!(bst.get(3) == Some(&"vvv"));
        assert!(bst.get(-2) == Some(&"vvvv"));
        assert!(bst.get(5) == Some(&"vvvvv"));
        assert!(bst.get(7) == Some(&"vvvvvvv"));

        assert!(bst.min() == bst.get(-2));
        assert!(bst.max() == bst.get(7));

        bst.put(-3, "min");
        bst.put(8, "max");

        assert!(bst.size() == 8);
        assert!(bst.min() == bst.get(-3));
        assert!(bst.max() == bst.get(8));
    }

    #[test]
    fn retrives_keys_with_inorder_ordering() {
        let mut bst = BST::Leaf;

        bst.put(0, "0");
        bst.put(-1, "vv");
        bst.put(3, "vvv");
        bst.put(-2, "vvvv");
        bst.put(5, "vvvvv");
        bst.put(7, "vvvvvvv");
        bst.put(-3, "min");
        bst.put(8, "max");

        let mut keys = bst.keys();

        for k in vec![-3, -2, -1, 0, 3, 5, 7, 8] {
            assert!(keys.dequeue() == Some(&k));
        }
    }

    #[test]
    fn retrives_values_with_inorder_ordering() {
        let mut bst = BST::Leaf;

        bst.put(0, "0");
        bst.put(-1, "-1");
        bst.put(3, "3");
        bst.put(-2, "-2");
        bst.put(5, "5");
        bst.put(7, "7");
        bst.put(-3, "-3");
        bst.put(8, "8");

        let mut values = bst.values();

        for v in vec!["-3", "-2", "-1", "0", "3", "5", "7", "8"] {
            assert!(values.dequeue() == Some(&v));
        }
    }
}
