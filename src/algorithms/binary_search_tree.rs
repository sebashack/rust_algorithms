use std::cmp::Ordering::{Equal, Greater, Less};

enum BST<K, V> {
    Node {
        key: K,
        val: V,
        left: Box<BST<K, V>>,
        right: Box<BST<K, V>>,
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
            } => match k.cmp(key) {
                Less => left.put(k, v),
                Greater => right.put(k, v),
                Equal => return,
            },
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
            } => match &k.cmp(key) {
                Less => left.get(k),
                Greater => right.get(k),
                Equal => Some(&val),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::binary_search_tree::BST;

    #[test]
    fn interface_operations_should_work_as_expected() {
        let mut bst = BST::Leaf;

        bst.put(0, "v");

        assert!(bst.get(0) == Some(&"v"));

        let mut bst = BST::<i32, &str>::new(0, "v");

        bst.put(-1, "vv");
        bst.put(3, "vvv");
        bst.put(-2, "vvvv");
        bst.put(5, "vvvvv");
        bst.put(7, "vvvvvvv");

        assert!(bst.get(0) == Some(&"v"));
        assert!(bst.get(-1) == Some(&"vv"));
        assert!(bst.get(3) == Some(&"vvv"));
        assert!(bst.get(-2) == Some(&"vvvv"));
        assert!(bst.get(5) == Some(&"vvvvv"));
        assert!(bst.get(7) == Some(&"vvvvvvv"));
    }
}
