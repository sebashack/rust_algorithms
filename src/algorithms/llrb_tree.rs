use std::cmp::Ordering::{Equal, Greater, Less};
use std::mem::MaybeUninit;
use std::mem::{replace, swap};

#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Black,
}

enum LLRBT<K, V> {
    Node {
        key: K,
        val: V,
        left: Box<LLRBT<K, V>>,
        right: Box<LLRBT<K, V>>,
        count: usize,
        color: Color,
    },
    Leaf,
}

impl<K, V> LLRBT<K, V>
where
    K: Ord,
{
    pub fn new(key: K, val: V) -> Self {
        use crate::algorithms::llrb_tree::Color::Black;

        LLRBT::Node {
            key,
            val,
            left: Box::new(LLRBT::Leaf),
            right: Box::new(LLRBT::Leaf),
            count: 1,
            color: Black,
        }
    }

    pub fn is_leaf(&self) -> bool {
        use crate::algorithms::llrb_tree::LLRBT::{Leaf, Node};

        if let Leaf = self {
            true
        } else {
            false
        }
    }

    pub fn is_red(&self) -> bool {
        use crate::algorithms::llrb_tree::Color::{Black, Red};
        use crate::algorithms::llrb_tree::LLRBT::{Leaf, Node};

        match self {
            Leaf => false,
            Node { color, .. } => match color {
                Red => true,
                Black => false,
            },
        }
    }

    pub fn size(&self) -> usize {
        use crate::algorithms::llrb_tree::LLRBT::{Leaf, Node};

        match self {
            Leaf => 0,
            Node { count, .. } => *count,
        }
    }

    pub fn get(&self, k: K) -> Option<&V> {
        use crate::algorithms::llrb_tree::LLRBT::{Leaf, Node};

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

    pub fn put(&mut self, k: K, v: V) {
        use crate::algorithms::llrb_tree::LLRBT::{Leaf, Node};

        match self {
            Leaf => *self = LLRBT::new(k, v),
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
            Node { left, right, .. } if right.is_red() && !left.is_red() => rotate_left(self),
            Node { left, right, .. } if left.is_red() && right.is_red() => flip_colors(self),
            Node {
                left: parent_left, ..
            } => {
                if let Node { left, .. } = &**parent_left {
                    if left.is_red() && parent_left.is_red() {
                        rotate_right(self)
                    }
                }
            }
            _ => {}
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

    pub fn min(&self) -> Option<&V> {
        use crate::algorithms::llrb_tree::LLRBT::{Leaf, Node};

        match self {
            Leaf => None,
            Node { val, left, .. } => {
                if left.is_leaf() {
                    Some(&val)
                } else {
                    left.min()
                }
            }
        }
    }

    pub fn max(&self) -> Option<&V> {
        use crate::algorithms::llrb_tree::LLRBT::{Leaf, Node};

        match self {
            Leaf => None,
            Node { val, right, .. } => {
                if right.is_leaf() {
                    Some(&val)
                } else {
                    right.max()
                }
            }
        }
    }
}

fn flip_colors<'a, K, V>(node: &'a mut LLRBT<K, V>)
where
    K: Ord,
{
    use crate::algorithms::llrb_tree::Color::{Black, Red};
    use crate::algorithms::llrb_tree::LLRBT::{Leaf, Node};

    assert!(!node.is_red());

    if let Node {
        key,
        val,
        left,
        right,
        count,
        color,
    } = node
    {
        assert!(left.is_red());
        assert!(right.is_red());

        *color = Red;

        if let Node { mut color, .. } = **left {
            color = Black;
        }

        if let Node { mut color, .. } = **right {
            color = Black;
        }
    }
}

fn rotate_left<'a, K, V>(parent: &'a mut LLRBT<K, V>)
where
    K: Ord,
{
    use crate::algorithms::llrb_tree::Color::{Black, Red};
    use crate::algorithms::llrb_tree::LLRBT::{Leaf, Node};

    if let Node {
        right,
        color: parent_color,
        ..
    } = parent
    {
        assert!(right.is_red());
        unsafe {
            let uninit_node = MaybeUninit::<Box<LLRBT<K, V>>>::uninit().assume_init();
            let x = replace(right, uninit_node);

            if let Node {
                mut left,
                mut color,
                ..
            } = *x
            {
                replace(&mut color, parent_color.clone());
                replace(parent_color, Red);
                swap(right, &mut left);
                swap(&mut *left, parent);
            }
        }
    }
}

fn rotate_right<'a, K, V>(parent: &'a mut LLRBT<K, V>)
where
    K: Ord,
{
    use crate::algorithms::llrb_tree::Color::{Black, Red};
    use crate::algorithms::llrb_tree::LLRBT::{Leaf, Node};

    if let Node {
        left,
        color: parent_color,
        ..
    } = parent
    {
        assert!(left.is_red());
        unsafe {
            let uninit_node = MaybeUninit::<Box<LLRBT<K, V>>>::uninit().assume_init();
            let x = replace(left, uninit_node);

            if let Node {
                mut right,
                mut color,
                ..
            } = *x
            {
                replace(&mut color, parent_color.clone());
                replace(parent_color, Red);
                swap(left, &mut right);
                swap(&mut *right, parent);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::llrb_tree::LLRBT;
    use crate::algorithms::llrb_tree::LLRBT::{Leaf, Node};

    #[test]
    fn interface_operations_should_work_as_expected() {
        let mut rbt = LLRBT::Leaf;
        assert!(rbt.size() == 0);

        rbt.put(0, "v");

        assert!(rbt.size() == 1);
        assert!(rbt.get(0) == Some(&"v"));

        let mut rbt = LLRBT::<i32, &str>::new(0, "v");

        rbt.put(-1, "vv");
        rbt.put(3, "vvv");
        rbt.put(-2, "vvvv");
        rbt.put(5, "vvvvv");
        rbt.put(7, "vvvvvvv");

        assert!(rbt.size() == 6);
        assert!(rbt.get(0) == Some(&"v"));
        assert!(rbt.get(-1) == Some(&"vv"));
        assert!(rbt.get(3) == Some(&"vvv"));
        assert!(rbt.get(-2) == Some(&"vvvv"));
        assert!(rbt.get(5) == Some(&"vvvvv"));
        assert!(rbt.get(7) == Some(&"vvvvvvv"));

        assert!(rbt.min() == rbt.get(-2));
        assert!(rbt.max() == rbt.get(7));

        rbt.put(-3, "min");
        rbt.put(8, "max");

        assert!(rbt.size() == 8);
        assert!(rbt.min() == rbt.get(-3));
        assert!(rbt.max() == rbt.get(8));
    }
}
